//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::fs;
use std::io::{self, Read, Write};
use std::str;

use anyhow::Context;
use clap::Parser;
use image::{io::Reader, ImageError, ImageFormat};
use qrcode::{bits::Bits, QrCode};
use rqrr::PreparedImage;

use crate::cli::{Command, InputFormat, Opt, OutputFormat};
use crate::{decode, encode};

/// Runs the program and returns the result.
#[allow(clippy::too_many_lines)]
pub fn run() -> anyhow::Result<()> {
    let args = Opt::parse();

    if let Some(shell) = args.generate_completion {
        Opt::print_completion(shell);
        return Ok(());
    }

    if let Some(command) = args.command {
        match command {
            Command::Encode {
                output,
                read_from,
                error_correction_level,
                symbol_version,
                margin,
                output_format,
                mode,
                variant,
                input,
            } => {
                let input = if let Some(string) = input {
                    string.into_bytes()
                } else if let Some(path) = read_from {
                    fs::read(&path)
                        .with_context(|| format!("Could not read data from {}", path.display()))?
                } else {
                    let mut buf = Vec::new();
                    io::stdin()
                        .read_to_end(&mut buf)
                        .context("Could not read data from stdin")?;
                    buf
                };

                let level = error_correction_level.into();
                let code = if let Some(version) = symbol_version {
                    let v = encode::set_version(version, &variant);
                    let mut bits = Bits::new(v);
                    encode::push_data_for_selected_mode(&mut bits, input, &mode)
                        .and_then(|_| bits.push_terminator(level))
                        .and_then(|_| QrCode::with_bits(bits, level))
                } else {
                    QrCode::with_error_correction_level(&input, level)
                }
                .context("Could not construct a QR code")?;

                match output_format {
                    format @ (OutputFormat::Svg | OutputFormat::Unicode) => {
                        let string = if format == OutputFormat::Svg {
                            encode::to_svg(&code, margin)
                        } else {
                            encode::to_unicode(&code, margin)
                        };

                        if let Some(file) = output {
                            fs::write(&file, string).with_context(|| {
                                format!("Could not write the image to {}", file.display())
                            })?;
                        } else {
                            println!("{string}");
                        }
                    }
                    format => {
                        let image = encode::to_image(&code, margin);

                        let format = ImageFormat::try_from(format)
                            .expect("The image format is not supported");
                        if let Some(file) = output {
                            image.save_with_format(&file, format).with_context(|| {
                                format!("Could not write the image to {}", file.display())
                            })?;
                        } else {
                            image
                                .write_to(&mut io::stdout(), format)
                                .context("Could not write the image to stdout")?;
                        }
                    }
                }
            }
            Command::Decode {
                input_format,
                input,
            } => {
                let input_format = if decode::is_svg(&input) {
                    Some(InputFormat::Svg)
                } else {
                    input_format
                };
                let image = match input_format {
                    Some(InputFormat::Svg) => decode::from_svg(&input),
                    Some(format) => decode::load_image_file(
                        &input,
                        format
                            .try_into()
                            .expect("The image format is not supported"),
                    )
                    .map_err(anyhow::Error::from),
                    _ => Reader::open(&input)
                        .and_then(Reader::with_guessed_format)
                        .map_err(ImageError::from)
                        .and_then(Reader::decode)
                        .map_err(anyhow::Error::from),
                }
                .with_context(|| format!("Could not read the image from {}", input.display()))?;
                let image = image.into_luma8();

                let mut image = PreparedImage::prepare(image);
                let grids = image.detect_grids();
                let contents =
                    decode::grids_as_bytes(grids).context("Could not decode the grid")?;

                for content in contents {
                    if let Ok(string) = str::from_utf8(&content.1) {
                        println!("{string}");
                    } else {
                        io::stdout()
                            .write_all(&content.1)
                            .context("Could not write data to stdout")?;
                    }
                }
            }
        }
    } else {
        unreachable!();
    }

    Ok(())
}
