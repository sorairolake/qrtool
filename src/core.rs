//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022-2023 Shun Sakai
//

use std::{
    fs,
    io::{self, Cursor, Read, Write},
    str,
};

use anyhow::Context;
use clap::Parser;
use image::{ImageError, ImageFormat};
use qrcode::{bits::Bits, QrCode};
use rqrr::PreparedImage;

use crate::{
    cli::{Command, Opt, OutputFormat},
    decode, encode,
    metadata::Extractor,
};

/// Runs the program and returns the result.
#[allow(clippy::too_many_lines)]
pub fn run() -> anyhow::Result<()> {
    let opt = Opt::parse();

    if let Some(shell) = opt.generate_completion {
        Opt::print_completion(shell);
        return Ok(());
    }

    if let Some(command) = opt.command {
        match command {
            Command::Encode(arg) => {
                let input = if let Some(string) = arg.input {
                    string.into_bytes()
                } else if let Some(path) = arg.read_from {
                    fs::read(&path)
                        .with_context(|| format!("could not read data from {}", path.display()))?
                } else {
                    let mut buf = Vec::new();
                    io::stdin()
                        .read_to_end(&mut buf)
                        .context("could not read data from stdin")?;
                    buf
                };

                let level = arg.error_correction_level.into();
                let code = if let Some(version) = arg.symbol_version {
                    let v = encode::set_version(version, &arg.variant)
                        .context("could not set the version")?;
                    let mut bits = Bits::new(v);
                    encode::push_data_for_selected_mode(&mut bits, input, &arg.mode)
                        .and_then(|_| bits.push_terminator(level))
                        .and_then(|_| QrCode::with_bits(bits, level))
                } else {
                    QrCode::with_error_correction_level(&input, level)
                }
                .context("could not construct a QR code")?;

                if arg.verbose {
                    let metadata = code.metadata();
                    eprintln!("Version: {}", metadata.symbol_version());
                    eprintln!("Level: {:?}", metadata.error_correction_level());
                }

                match arg.output_format {
                    format @ (OutputFormat::Svg | OutputFormat::Terminal) => {
                        let string = if format == OutputFormat::Svg {
                            encode::to_svg(&code, arg.margin, &(arg.foreground, arg.background))
                        } else {
                            encode::to_terminal(&code, arg.margin)
                        };

                        if let Some(file) = arg.output {
                            fs::write(&file, string).with_context(|| {
                                format!("could not write the image to {}", file.display())
                            })?;
                        } else {
                            println!("{string}");
                        }
                    }
                    format => {
                        let image =
                            encode::to_image(&code, arg.margin, &(arg.foreground, arg.background));

                        let format = ImageFormat::try_from(format)
                            .expect("the image format is not supported");
                        if let Some(file) = arg.output {
                            image.save_with_format(&file, format).with_context(|| {
                                format!("could not write the image to {}", file.display())
                            })?;
                        } else {
                            let mut buf = Vec::new();
                            image
                                .write_to(&mut Cursor::new(&mut buf), format)
                                .and_then(|_| {
                                    io::stdout().write_all(&buf).map_err(ImageError::from)
                                })
                                .context("could not write the image to stdout")?;
                        }
                    }
                }
            }
            Command::Decode(arg) => {
                let input_format = arg.input_format;
                #[cfg(feature = "decode-from-svg")]
                #[allow(clippy::option_if_let_else)]
                let input_format = match arg.input {
                    Some(ref path) if decode::is_svg(path) => Some(crate::cli::InputFormat::Svg),
                    _ => input_format,
                };
                let input = match arg.input {
                    Some(path) if path.to_str().unwrap_or_default() != "-" => fs::read(&path)
                        .with_context(|| format!("could not read data from {}", path.display()))?,
                    _ => {
                        let mut buf = Vec::new();
                        io::stdin()
                            .read_to_end(&mut buf)
                            .context("could not read data from stdin")?;
                        buf
                    }
                };
                #[allow(clippy::option_if_let_else)]
                let image = match input_format {
                    #[cfg(feature = "decode-from-svg")]
                    Some(crate::cli::InputFormat::Svg) => decode::from_svg(&input),
                    Some(format) => image::load_from_memory_with_format(
                        &input,
                        format
                            .try_into()
                            .expect("the image format is not supported"),
                    )
                    .map_err(anyhow::Error::from),
                    _ => image::load_from_memory(&input).map_err(anyhow::Error::from),
                }
                .context("could not read the image")?;
                let image = image.into_luma8();

                let mut image = PreparedImage::prepare(image);
                let grids = image.detect_grids();
                let contents =
                    decode::grids_as_bytes(grids).context("could not decode the grid")?;

                for content in contents {
                    if arg.verbose || arg.metadata {
                        let metadata = content.0.metadata();
                        eprintln!("Version: {}", metadata.symbol_version());
                        eprintln!("Level: {:?}", metadata.error_correction_level());
                        if arg.metadata {
                            continue;
                        }
                    }

                    if let Ok(string) = str::from_utf8(&content.1) {
                        print!("{string}");
                    } else {
                        io::stdout()
                            .write_all(&content.1)
                            .context("could not write data to stdout")?;
                    }
                }
            }
        }
    } else {
        unreachable!();
    }
    Ok(())
}
