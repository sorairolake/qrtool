// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    io::{self, Write},
    num::NonZeroU32,
    path::PathBuf,
};

use clap::{value_parser, Args, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use clap_complete::Generator;
use csscolorparser::Color;
use image::{ImageError, ImageFormat};

const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\n",
    include_str!("assets/long-version.md")
);

const AFTER_LONG_HELP: &str = include_str!("assets/after-long-help.md");

const ENCODE_AFTER_LONG_HELP: &str = include_str!("assets/encode-after-long-help.md");

const DECODE_AFTER_LONG_HELP: &str = include_str!("assets/decode-after-long-help.md");

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[command(
    version,
    long_version(LONG_VERSION),
    about,
    max_term_width(100),
    propagate_version(true),
    after_long_help(AFTER_LONG_HELP),
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
    #[command(
        after_long_help(ENCODE_AFTER_LONG_HELP),
        visible_alias("enc"),
        visible_alias("e")
    )]
    Encode(Encode),

    /// Detect and decode a QR code.
    #[command(
        after_long_help(DECODE_AFTER_LONG_HELP),
        visible_alias("dec"),
        visible_alias("d")
    )]
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

    /// The module size in pixels.
    ///
    /// If this option is not specified, the module size is 8 when the output
    /// format is PNG or SVG, and 1 when the output format is UTF-8 string.
    #[arg(short, long, value_name("NUMBER"))]
    pub size: Option<NonZeroU32>,

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
    /// If this option is not specified, the minimum version required to store
    /// the data will be automatically chosen. For normal QR code, <NUMBER>
    /// should be between 1 and 40. For Micro QR code, <NUMBER> should be
    /// between 1 and 4.
    #[arg(
        value_parser(value_parser!(i16).range(1..=40)),
        short('v'),
        long,
        visible_alias("symversion"),
        value_name("NUMBER")
    )]
    pub symbol_version: Option<i16>,

    /// The width of margin.
    ///
    /// If this option is not specified, the margin will be 4 for normal QR code
    /// and 2 for Micro QR code.
    #[arg(short, long, value_name("NUMBER"))]
    pub margin: Option<u32>,

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
    ///
    /// If this option is not specified, use the optimal encoding.
    #[arg(
        long,
        value_enum,
        requires("symbol_version"),
        value_name("MODE"),
        ignore_case(true)
    )]
    pub mode: Option<Mode>,

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
    /// <COLOR> takes a CSS color string. Colored output is only available when
    /// the output format is PNG or SVG.
    #[arg(long, default_value("black"), value_name("COLOR"))]
    pub foreground: Color,

    /// Background color.
    ///
    /// <COLOR> takes a CSS color string. Colored output is only available when
    /// the output format is PNG or SVG.
    #[arg(long, default_value("white"), value_name("COLOR"))]
    pub background: Color,

    /// Also print the metadata.
    ///
    /// It is output to stderr.
    #[arg(long)]
    pub verbose: bool,

    /// Input data.
    ///
    /// If [STRING] is not specified, data will be read from stdin. [STRING]
    /// must be a valid UTF-8 string.
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
    /// read from stdin. Supported raster image formats are based on the formats
    /// supported by the image crate. The format of [IMAGE] is determined based
    /// on the extension or the magic number if possible. If the format cannot
    /// be determined, use '--type'. Note that the SVG image is rasterized
    /// before scanning.
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

    #[allow(clippy::enum_variant_names)]
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

impl From<Ecc> for qrcode::EcLevel {
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

#[derive(Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum Mode {
    /// All digits.
    Numeric,

    /// Alphanumerics and few symbols.
    Alphanumeric,

    /// Arbitrary binary data.
    Byte,

    /// Shift JIS text.
    Kanji,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, ValueEnum)]
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

    #[test]
    fn file_name_shell() {
        assert_eq!(Shell::Bash.file_name("qrtool"), "qrtool.bash");
        assert_eq!(Shell::Elvish.file_name("qrtool"), "qrtool.elv");
        assert_eq!(Shell::Fish.file_name("qrtool"), "qrtool.fish");
        assert_eq!(Shell::Nushell.file_name("qrtool"), "qrtool.nu");
        assert_eq!(Shell::PowerShell.file_name("qrtool"), "_qrtool.ps1");
        assert_eq!(Shell::Zsh.file_name("qrtool"), "_qrtool");
    }

    #[test]
    fn default_ecc() {
        assert_eq!(Ecc::default(), Ecc::M);
    }

    #[test]
    fn from_ecc_to_ec_level() {
        assert_eq!(qrcode::EcLevel::from(Ecc::L), qrcode::EcLevel::L);
        assert_eq!(qrcode::EcLevel::from(Ecc::M), qrcode::EcLevel::M);
        assert_eq!(qrcode::EcLevel::from(Ecc::Q), qrcode::EcLevel::Q);
        assert_eq!(qrcode::EcLevel::from(Ecc::H), qrcode::EcLevel::H);
    }

    #[test]
    fn default_output_format() {
        assert_eq!(OutputFormat::default(), OutputFormat::Png);
    }

    #[test]
    fn default_variant() {
        assert_eq!(Variant::default(), Variant::Normal);
    }

    #[test]
    fn try_from_input_format_to_image_format() {
        assert_eq!(
            ImageFormat::try_from(InputFormat::Bmp).unwrap(),
            ImageFormat::Bmp
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Dds).unwrap(),
            ImageFormat::Dds
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Farbfeld).unwrap(),
            ImageFormat::Farbfeld
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Gif).unwrap(),
            ImageFormat::Gif
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Hdr).unwrap(),
            ImageFormat::Hdr
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Ico).unwrap(),
            ImageFormat::Ico
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Jpeg).unwrap(),
            ImageFormat::Jpeg
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::OpenExr).unwrap(),
            ImageFormat::OpenExr
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Png).unwrap(),
            ImageFormat::Png
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Pnm).unwrap(),
            ImageFormat::Pnm
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Qoi).unwrap(),
            ImageFormat::Qoi
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Tga).unwrap(),
            ImageFormat::Tga
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Tiff).unwrap(),
            ImageFormat::Tiff
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::WebP).unwrap(),
            ImageFormat::WebP
        );
    }

    #[cfg(feature = "decode-from-svg")]
    #[test]
    fn try_from_input_format_to_image_format_when_svg() {
        assert!(ImageFormat::try_from(InputFormat::Svg).is_err());
    }
}
