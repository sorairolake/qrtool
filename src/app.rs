// SPDX-FileCopyrightText: 2023 Shun Sakai
// SPDX-FileCopyrightText: 2024 Alexis Hildebrandt
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    fs,
    io::{self, Cursor, Read, Write},
    num::NonZeroU32,
    str,
};

use anyhow::Context;
use clap::Parser;
use image::ImageFormat;
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
    let opt = Opt::parse().validate()?;

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
                    if let Some(mode) = arg.mode {
                        encode::push_data_for_selected_mode(&mut bits, input, &mode)
                    } else {
                        bits.push_optimal_data(&input)
                    }
                    .and_then(|()| bits.push_terminator(level))
                    .and_then(|()| QrCode::with_bits(bits, level))
                } else {
                    QrCode::with_error_correction_level(&input, level)
                }
                .context("could not construct a QR code")?;

                if arg.verbose {
                    let metadata = code.metadata();
                    eprintln!("Version: {}", metadata.symbol_version());
                    eprintln!("Level: {:?}", metadata.error_correction_level());
                }

                let margin =
                    arg.margin
                        .unwrap_or_else(|| if code.version().is_micro() { 2 } else { 4 });
                let module_size = arg.size.map(NonZeroU32::get);
                match arg.output_format {
                    format @ (OutputFormat::Pic | OutputFormat::Svg | OutputFormat::Terminal) => {
                        let string = match format {
                            OutputFormat::Pic => encode::to_pic(&code, margin, module_size),
                            OutputFormat::Svg => encode::to_svg(
                                &code,
                                margin,
                                &(arg.foreground, arg.background),
                                module_size,
                            ),
                            OutputFormat::Terminal => {
                                encode::to_terminal(&code, margin, module_size)
                            }
                            OutputFormat::Png => unreachable!(),
                        };

                        if let Some(file) = arg.output {
                            fs::write(&file, string).with_context(|| {
                                format!("could not write the image to {}", file.display())
                            })?;
                        } else {
                            println!("{string}");
                        }
                    }
                    OutputFormat::Png => {
                        let image = encode::to_image(
                            &code,
                            margin,
                            &(arg.foreground, arg.background),
                            module_size,
                        );
                        let mut buf = Vec::new();
                        image
                            .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
                            .context("could not write the image to the buffer")?;

                        #[cfg(feature = "optimize-output-png")]
                        if let Some(level) = arg.optimize_png {
                            let mut optimize_opt = oxipng::Options::from_preset(level.into());
                            if let Some(iterations) = arg.zopfli {
                                optimize_opt.deflate = oxipng::Deflaters::Zopfli { iterations };
                            }
                            buf = oxipng::optimize_from_memory(&buf, &optimize_opt)
                                .context("could not optimize the image")?;
                        }

                        if let Some(file) = arg.output {
                            fs::write(&file, buf).with_context(|| {
                                format!("could not write the image to {}", file.display())
                            })?;
                        } else {
                            io::stdout()
                                .write_all(&buf)
                                .context("could not write the image to stdout")?;
                        }
                    }
                }
            }
            Command::Decode(arg) => {
                let input_format = arg.input_format;
                #[cfg(feature = "decode-from-svg")]
                #[allow(clippy::option_if_let_else)]
                let input_format = match (&input_format, &arg.input) {
                    (None, Some(path)) if decode::is_svg(path) => {
                        Some(crate::cli::InputFormat::Svg)
                    }
                    _ => input_format,
                };
                let input = match arg.input {
                    Some(ref path) if path.to_str().unwrap_or_default() != "-" => fs::read(path)
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
                    format => {
                        let format = if let Some(f) = format {
                            f.try_into()
                        } else {
                            image::guess_format(&input).or_else(|err| {
                                arg.input.map_or_else(|| Err(err), ImageFormat::from_path)
                            })
                        }
                        .context("could not determine the image format")?;
                        image::load_from_memory_with_format(&input, format)
                            .map_err(anyhow::Error::from)
                    }
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
