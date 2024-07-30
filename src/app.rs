// SPDX-FileCopyrightText: 2023 Shun Sakai
// SPDX-FileCopyrightText: 2024 Alexis Hildebrandt
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    fs::{self, File},
    io::{self, BufReader, Cursor, Read, Write},
    num::NonZeroU32,
};

use anyhow::Context;
use clap::Parser;
use image::ImageFormat;
use qrcode::{bits::Bits, QrCode};
use rqrr::PreparedImage;

use crate::{
    cli::{Command, Opt, OutputFormat},
    decode, encode,
    input::Input,
    metadata::Extractor,
};

const MAX_DATA_SIZE: u64 = 7089;

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
                    Input::String(Cursor::new(string))
                } else if let Some(ref path) = arg.read_from {
                    let f = File::open(path)
                        .with_context(|| format!("could not open {}", path.display()))?;
                    Input::File(f)
                } else {
                    Input::Stdin(io::stdin())
                };
                let reader = BufReader::new(input);
                let mut buf = Vec::new();
                reader
                    .take(MAX_DATA_SIZE + 1)
                    .read_to_end(&mut buf)
                    .context("could not read data")?;

                let level = arg.error_correction_level.into();
                let code = if let Some(version) = arg.symbol_version {
                    let v = encode::set_version(version, &arg.variant)
                        .context("could not set the version")?;
                    let mut bits = Bits::new(v);
                    if let Some(mode) = arg.mode {
                        encode::push_data_for_selected_mode(&mut bits, buf, &mode)
                    } else {
                        bits.push_optimal_data(&buf)
                    }
                    .and_then(|()| bits.push_terminator(level))
                    .and_then(|()| QrCode::with_bits(bits, level))
                } else {
                    QrCode::with_error_correction_level(&buf, level)
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
                let is_invert = matches!(
                    arg.output_format,
                    OutputFormat::AsciiInvert | OutputFormat::UnicodeInvert
                );
                let output = match arg.output_format {
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
                        buf
                    }
                    OutputFormat::Svg => encode::to_svg(
                        &code,
                        margin,
                        &(arg.foreground, arg.background),
                        module_size,
                    )
                    .into(),
                    OutputFormat::Pic => encode::to_pic(&code, margin, module_size).into(),
                    #[cfg(feature = "output-as-ansi")]
                    OutputFormat::Ansi => encode::to_ansi(
                        &code,
                        margin,
                        &(arg.foreground, arg.background),
                        module_size,
                    )
                    .into(),
                    #[cfg(feature = "output-as-ansi")]
                    OutputFormat::Ansi256 => encode::to_ansi_256(
                        &code,
                        margin,
                        &(arg.foreground, arg.background),
                        module_size,
                    )
                    .into(),
                    #[cfg(feature = "output-as-ansi")]
                    OutputFormat::AnsiTrueColor => encode::to_ansi_true_color(
                        &code,
                        margin,
                        &(arg.foreground, arg.background),
                        module_size,
                    )
                    .into(),
                    OutputFormat::Ascii | OutputFormat::AsciiInvert => {
                        encode::to_ascii(&code, margin, module_size, is_invert).into()
                    }
                    OutputFormat::Unicode | OutputFormat::UnicodeInvert => {
                        encode::to_unicode(&code, margin, module_size, is_invert).into()
                    }
                };

                if let Some(file) = arg.output {
                    fs::write(&file, output).with_context(|| {
                        format!("could not write the image to {}", file.display())
                    })?;
                } else {
                    io::stdout()
                        .write_all(&output)
                        .context("could not write the image to stdout")?;
                }
            }
            Command::Decode(arg) => {
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
                let input_format = arg.input_format;
                #[cfg(feature = "decode-from-svg")]
                let input_format = input_format
                    .or_else(|| is_svg::is_svg(&input).then_some(crate::cli::InputFormat::Svg));
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

                    io::stdout()
                        .write_all(&content.1)
                        .context("could not write data to stdout")?;
                }
            }
        }
    } else {
        unreachable!();
    }
    Ok(())
}
