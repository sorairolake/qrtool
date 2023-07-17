//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022-2023 Shun Sakai
//

use std::{
    io::{self, Write},
    path::PathBuf,
};

use clap::{value_parser, Args, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use clap_complete::Generator;
use csscolorparser::Color;
use image::{ImageError, ImageFormat};

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
    /// For normal QR code, <NUMBER> should be between 1 and 40.
    /// For Micro QR code, <NUMBER> should be between 1 and 4.
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
    /// <COLOR> takes a CSS color string.
    #[arg(long, default_value("black"), value_name("COLOR"))]
    pub foreground: Color,

    /// Background color.
    ///
    /// <COLOR> takes a CSS color string.
    #[arg(long, default_value("white"), value_name("COLOR"))]
    pub background: Color,

    /// Also print the metadata.
    ///
    /// It is output to stderr.
    #[arg(long)]
    pub verbose: bool,

    /// Input data.
    ///
    /// If [STRING] is not specified, data will be read from stdin.
    /// [STRING] must be a valid UTF-8 string.
    #[arg(value_name("STRING"))]
    pub input: Option<String>,
}

#[derive(Args, Debug)]
pub struct Decode {
    /// The format of the input.
    ///
    /// If <FORMAT> is not specified, the format is determined based on the
    /// extension or the magic number.
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
    /// If [IMAGE] is not specified, or if "-" is specified, the image will be
    /// read from stdin. Supported raster image formats are based on the
    /// formats supported by the image crate. The format of [IMAGE] is
    /// determined based on the extension or the magic number if possible.
    /// If the format cannot be determined, use '--type'.
    /// Note that the SVG image is rasterized before scanning.
    #[arg(value_name("IMAGE"), value_hint(ValueHint::FilePath))]
    pub input: Option<PathBuf>,
}

impl Opt {
    /// Generates shell completion and print it.
    pub fn print_completion(gen: impl Generator) {
        clap_complete::generate(
            gen,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "lower")]
pub enum Shell {
    /// Bash.
    Bash,

    /// Elvish.
    Elvish,

    /// fish.
    Fish,

    /// Nushell.
    Nushell,

    /// PowerShell.
    PowerShell,

    /// Zsh.
    Zsh,
}

impl Generator for Shell {
    fn file_name(&self, name: &str) -> String {
        match self {
            Self::Bash => clap_complete::Shell::Bash.file_name(name),
            Self::Elvish => clap_complete::Shell::Elvish.file_name(name),
            Self::Fish => clap_complete::Shell::Fish.file_name(name),
            Self::Nushell => clap_complete_nushell::Nushell.file_name(name),
            Self::PowerShell => clap_complete::Shell::PowerShell.file_name(name),
            Self::Zsh => clap_complete::Shell::Zsh.file_name(name),
        }
    }

    fn generate(&self, cmd: &clap::Command, buf: &mut dyn Write) {
        match self {
            Self::Bash => clap_complete::Shell::Bash.generate(cmd, buf),
            Self::Elvish => clap_complete::Shell::Elvish.generate(cmd, buf),
            Self::Fish => clap_complete::Shell::Fish.generate(cmd, buf),
            Self::Nushell => clap_complete_nushell::Nushell.generate(cmd, buf),
            Self::PowerShell => clap_complete::Shell::PowerShell.generate(cmd, buf),
            Self::Zsh => clap_complete::Shell::Zsh.generate(cmd, buf),
        }
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

    /// Graphics Interchange Format.
    Gif,

    /// Radiance RGBE.
    Hdr,

    /// ICO file format.
    ///
    /// This value also includes the CUR file format.
    Ico,

    /// JPEG.
    Jpeg,

    /// OpenEXR.
    OpenExr,

    /// Portable Network Graphics.
    Png,

    /// Portable Anymap Format.
    Pnm,

    /// Quite OK Image Format.
    Qoi,

    /// Scalable Vector Graphics.
    ///
    /// This value also includes the gzip-compressed SVG image.
    #[cfg(feature = "decode-from-svg")]
    Svg,

    /// Truevision TGA.
    Tga,

    /// Tag Image File Format.
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
            InputFormat::Qoi => Ok(Self::Qoi),
            #[cfg(feature = "decode-from-svg")]
            InputFormat::Svg => Err(Self::Error::Unsupported(
                image::error::ImageFormatHint::Unknown.into(),
            )),
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
