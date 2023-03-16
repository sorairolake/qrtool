//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022-2023 Shun Sakai
//

use std::{io, path::PathBuf};

use clap::{value_parser, Args, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use clap_complete::{Generator, Shell};
use image::{error::ImageFormatHint, ImageError, ImageFormat};

use crate::color::Color;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[command(
    version,
    about,
    max_term_width(100),
    propagate_version(true),
    arg_required_else_help(true),
    args_conflicts_with_subcommands(true)
)]
pub struct Opt {
    /// Generate shell completion.
    ///
    /// The completion is output to stdout.
    #[arg(long, value_enum, value_name("SHELL"))]
    pub generate_completion: Option<Shell>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Encode input data in a QR code.
    Encode(Encode),

    /// Detect and decode a QR code.
    Decode(Decode),
}

#[derive(Args, Debug)]
pub struct Encode {
    /// Output the result to a file.
    #[arg(short, long, value_name("FILE"))]
    pub output: Option<PathBuf>,

    /// Read input data from a file.
    #[arg(
        short,
        long,
        value_name("FILE"),
        value_hint(ValueHint::FilePath),
        conflicts_with("input")
    )]
    pub read_from: Option<PathBuf>,

    /// Error correction level.
    #[arg(
        short('l'),
        long,
        value_enum,
        default_value_t,
        visible_alias("level"),
        value_name("LEVEL"),
        ignore_case(true)
    )]
    pub error_correction_level: Ecc,

    /// The version of the symbol.
    ///
    /// For normal QR code, it should be between 1 and 40.
    /// For Micro QR code, it should be between 1 and 4.
    #[arg(
        value_parser(value_parser!(i16).range(1..=40)),
        short('v'),
        long,
        visible_alias("symversion"),
        value_name("NUMBER")
    )]
    pub symbol_version: Option<i16>,

    /// The width of margin.
    #[arg(short, long, default_value("4"), value_name("NUMBER"))]
    pub margin: u32,

    /// The format of the output.
    #[arg(
        short('t'),
        long("type"),
        value_enum,
        default_value_t,
        value_name("FORMAT"),
        ignore_case(true)
    )]
    pub output_format: OutputFormat,

    /// The mode of the output.
    #[arg(
        long,
        value_enum,
        default_value_t,
        value_name("MODE"),
        ignore_case(true)
    )]
    pub mode: Mode,

    /// The type of QR code.
    #[arg(
        long,
        value_enum,
        default_value_t,
        requires("symbol_version"),
        value_name("TYPE"),
        ignore_case(true)
    )]
    pub variant: Variant,

    /// Foreground color.
    ///
    /// It takes hexadecimal notation such as RRGGBB (hex triplet) or RRGGBBAA
    /// and shorthands of these. A leading number sign is allowed.
    #[arg(long, default_value("#000000"), value_name("COLOR"))]
    pub foreground: Color,

    /// Background color.
    ///
    /// It takes hexadecimal notation such as RRGGBB (hex triplet) or RRGGBBAA
    /// and shorthands of these. A leading number sign is allowed.
    #[arg(long, default_value("#ffffff"), value_name("COLOR"))]
    pub background: Color,

    /// Also print the metadata.
    ///
    /// It is output to stderr.
    #[arg(long)]
    pub verbose: bool,

    /// Input data.
    ///
    /// If it is not specified, data will be read from stdin.
    /// It takes a valid UTF-8 string.
    #[arg(value_name("STRING"))]
    pub input: Option<String>,
}

#[derive(Args, Debug)]
pub struct Decode {
    /// The format of the input.
    ///
    /// If it is not specified, the format will be guessed based on the
    /// extension, and the raster format will use the content in addition to it.
    #[arg(
        short('t'),
        long("type"),
        value_enum,
        value_name("FORMAT"),
        ignore_case(true)
    )]
    pub input_format: Option<InputFormat>,

    /// Also print the metadata.
    ///
    /// It is output to stderr.
    #[arg(long, conflicts_with("metadata"))]
    pub verbose: bool,

    /// Print only the metadata.
    ///
    /// It is output to stderr.
    #[arg(long)]
    pub metadata: bool,

    /// Input image file.
    ///
    /// If it is not specified, or if "-" is specified, the image will be read
    /// from stdin. Supported raster image formats are any formats supported
    /// by the image crate. The format guess based on the extension, and the
    /// raster format use the content in addition to it. Note that the SVG
    /// image is rasterized before scanning.
    #[arg(value_name("IMAGE"), value_hint(ValueHint::FilePath))]
    pub input: Option<PathBuf>,
}

impl Opt {
    /// Generate shell completion and print it.
    pub fn print_completion(gen: impl Generator) {
        clap_complete::generate(
            gen,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum Ecc {
    /// Level L.
    ///
    /// 7% of codewords can be restored.
    L,

    /// Level M.
    ///
    /// 15% of codewords can be restored.
    #[default]
    M,

    /// Level Q.
    ///
    /// 25% of codewords can be restored.
    Q,

    /// Level H.
    ///
    /// 30% of codewords can be restored.
    H,
}

impl From<Ecc> for qrencode::EcLevel {
    fn from(level: Ecc) -> Self {
        match level {
            Ecc::L => Self::L,
            Ecc::M => Self::M,
            Ecc::Q => Self::Q,
            Ecc::H => Self::H,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    /// Portable Network Graphics.
    ///
    /// This outputs 32-bit RGBA PNG image.
    #[default]
    Png,

    /// Scalable Vector Graphics.
    Svg,

    /// To the terminal as UTF-8 string.
    Terminal,
}

impl TryFrom<OutputFormat> for ImageFormat {
    type Error = ImageError;

    fn try_from(format: OutputFormat) -> Result<Self, Self::Error> {
        match format {
            OutputFormat::Png => Ok(Self::Png),
            _ => Err(Self::Error::Unsupported(ImageFormatHint::Unknown.into())),
        }
    }
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Mode {
    /// All digits.
    Numeric,

    /// Alphanumerics and few symbols.
    Alphanumeric,

    /// Arbitrary binary data.
    #[default]
    Byte,

    /// Shift JIS text.
    Kanji,
}

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Variant {
    /// Normal QR code.
    #[default]
    Normal,

    /// Micro QR code.
    Micro,
}

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "lower")]
pub enum InputFormat {
    /// Windows Bitmap.
    Bmp,

    /// DirectDraw Surface.
    Dds,

    /// Farbfeld.
    Farbfeld,

    /// GIF.
    Gif,

    /// Radiance RGBE.
    Hdr,

    /// ICO.
    Ico,

    /// JPEG.
    Jpeg,

    /// OpenEXR.
    OpenExr,

    /// Portable Network Graphics.
    Png,

    /// PNM.
    Pnm,

    /// Scalable Vector Graphics.
    ///
    /// This also includes gzipped it.
    #[cfg(feature = "decode-from-svg")]
    Svg,

    /// Truevision TGA.
    Tga,

    /// TIFF.
    Tiff,

    /// WebP.
    WebP,
}

impl TryFrom<InputFormat> for ImageFormat {
    type Error = ImageError;

    fn try_from(format: InputFormat) -> Result<Self, Self::Error> {
        match format {
            InputFormat::Bmp => Ok(Self::Bmp),
            InputFormat::Dds => Ok(Self::Dds),
            InputFormat::Farbfeld => Ok(Self::Farbfeld),
            InputFormat::Gif => Ok(Self::Gif),
            InputFormat::Hdr => Ok(Self::Hdr),
            InputFormat::Ico => Ok(Self::Ico),
            InputFormat::Jpeg => Ok(Self::Jpeg),
            InputFormat::OpenExr => Ok(Self::OpenExr),
            InputFormat::Png => Ok(Self::Png),
            InputFormat::Pnm => Ok(Self::Pnm),
            #[cfg(feature = "decode-from-svg")]
            InputFormat::Svg => Err(Self::Error::Unsupported(ImageFormatHint::Unknown.into())),
            InputFormat::Tga => Ok(Self::Tga),
            InputFormat::Tiff => Ok(Self::Tiff),
            InputFormat::WebP => Ok(Self::WebP),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        Opt::command().debug_assert();
    }
}
