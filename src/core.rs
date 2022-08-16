//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, BufReader, Cursor, Read, Write};
use std::path::Path;
use std::str;

use clap::Parser;

use crate::cli::{Command, InputFormat, Mode, Opt, OutputFormat, Variant};

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
                    fs::read(path)?
                } else {
                    let mut buf = Vec::new();
                    io::stdin().read_to_end(&mut buf)?;

                    buf
                };

                let level = error_correction_level.into();

                let code = if let Some(version) = symbol_version {
                    let v = match variant {
                        Variant::Normal => qrcode::Version::Normal(version),
                        Variant::Micro => qrcode::Version::Micro(version),
                    };

                    let mut bits = qrcode::bits::Bits::new(v);
                    match mode {
                        Mode::Numeric => bits.push_numeric_data(&input)?,
                        Mode::Alphanumeric => bits.push_alphanumeric_data(&input)?,
                        Mode::Byte => bits.push_byte_data(&input)?,
                        Mode::Kanji => bits.push_kanji_data(&input)?,
                    }
                    bits.push_terminator(level)?;

                    qrcode::QrCode::with_bits(bits, level)?
                } else {
                    qrcode::QrCode::with_error_correction_level(&input, level)?
                };

                match output_format {
                    format @ (OutputFormat::Svg | OutputFormat::Unicode) => {
                        let string = if format == OutputFormat::Svg {
                            qrcode::render::Renderer::<qrcode::render::svg::Color<'_>>::new(
                                &code.to_colors(),
                                code.width(),
                                margin,
                            )
                            .build()
                        } else {
                            qrcode::render::Renderer::<qrcode::render::unicode::Dense1x2>::new(
                                &code.to_colors(),
                                code.width(),
                                margin,
                            )
                            .build()
                        };

                        if let Some(file) = output {
                            fs::write(file, string)?;
                        } else {
                            println!("{string}");
                        }
                    }
                    format => {
                        let format = image::ImageFormat::try_from(format)?;

                        let image = qrcode::render::Renderer::<image::Luma<u8>>::new(
                            &code.to_colors(),
                            code.width(),
                            margin,
                        )
                        .build();

                        if let Some(file) = output {
                            image.save_with_format(file, format)?;
                        } else {
                            image::DynamicImage::ImageLuma8(image)
                                .write_to(&mut io::stdout(), format)?;
                        }
                    }
                }
            }
            Command::Decode {
                input_format,
                input,
            } => {
                let input_format =
                    if let Some("svg" | "svgz") = input.extension().and_then(OsStr::to_str) {
                        Some(InputFormat::Svg)
                    } else {
                        input_format
                    };

                let image = match input_format {
                    Some(InputFormat::Svg) => {
                        let opt = usvg::Options {
                            resources_dir: input
                                .canonicalize()
                                .ok()
                                .and_then(|path| path.parent().map(Path::to_path_buf)),
                            ..Default::default()
                        };

                        let image = fs::read(input)?;
                        let tree = usvg::Tree::from_data(&image, &opt.to_ref())?;

                        let pixmap_size = tree.svg_node().size.to_screen_size();
                        let mut pixmap =
                            tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
                                .unwrap();

                        resvg::render(
                            &tree,
                            usvg::FitTo::Original,
                            tiny_skia::Transform::default(),
                            pixmap.as_mut(),
                        )
                        .unwrap();

                        let png = pixmap.encode_png()?;

                        image::io::Reader::with_format(Cursor::new(png), image::ImageFormat::Png)
                            .decode()?
                    }
                    Some(format) => {
                        let reader =
                            BufReader::new(File::open(input).map_err(image::ImageError::IoError)?);

                        image::load(reader, format.try_into()?)?
                    }
                    _ => image::io::Reader::open(input)?
                        .with_guessed_format()?
                        .decode()?,
                };
                let image = image.into_luma8();

                let mut image = rqrr::PreparedImage::prepare(image);
                let grids = image.detect_grids();

                let contents: Result<Vec<(rqrr::MetaData, Vec<u8>)>, rqrr::DeQRError> = grids
                    .into_iter()
                    .map(|grid| {
                        let mut writer = Vec::new();
                        grid.decode_to(&mut writer).map(|meta| (meta, writer))
                    })
                    .collect();
                let contents = contents?;

                for content in contents {
                    if let Ok(string) = str::from_utf8(&content.1) {
                        println!("{string}");
                    } else {
                        io::stdout().write_all(&content.1)?;
                    }
                }
            }
        }
    } else {
        unreachable!();
    }

    Ok(())
}
