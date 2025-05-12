// SPDX-FileCopyrightText: 2022 Shun Sakai
// SPDX-FileCopyrightText: 2024 Alexis Hildebrandt
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{
    io::{self, Write},
    num::NonZeroU32,
    path::PathBuf,
};

use anyhow::anyhow;
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint, value_parser};
use clap_complete::Generator;
use csscolorparser::Color;
use image::{ImageError, ImageFormat};

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    max_term_width(100),
    propagate_version(true),
    arg_required_else_help(false)
)]
pub struct Opt {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Encode input data in a QR code.
    ///
    /// By default, the result will be output to standard output.
    #[command(visible_alias("enc"), visible_alias("e"))]
    Encode(Encode),

    /// Detect and decode a QR code.
    ///
    /// By default, the result will be output to standard output.
    #[command(visible_alias("dec"), visible_alias("d"))]
    Decode(Decode),

    /// Generate shell completion.
    ///
    /// The completion is output to standard output.
    Completion(Completion),
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
    /// format is PNG or SVG, and 1 otherwise.
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

    /// Set the optimization level for a PNG image.
    ///
    /// Lower levels are faster, higher levels provide better compression. If
    /// <LEVEL> is not specified, it is assumed that the default level 2 is
    /// specified.
    #[cfg(feature = "optimize-output-png")]
    #[arg(
        long,
        value_enum,
        num_args(0..=1),
        value_name("LEVEL"),
        ignore_case(true),
        default_missing_value("2")
    )]
    pub optimize_png: Option<PngOptimizationLevel>,

    /// Use Zopfli to compress PNG image.
    ///
    /// Perform compression for the number of iterations specified by
    /// <ITERATION>. If <ITERATION> is not specified, it is assumed that 15 is
    /// specified as the number of iterations.
    #[cfg(feature = "optimize-output-png")]
    #[arg(
        long,
        requires("optimize_png"),
        num_args(0..=1),
        value_name("ITERATION"),
        default_missing_value("15")
    )]
    pub zopfli: Option<std::num::NonZeroU8>,

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
    /// the output format is PNG, SVG or any ANSI escape sequences. Note that
    /// lossy conversion may be performed depending on the color space supported
    /// by the method to specify a color, the color depth supported by the
    /// output format, etc.
    #[arg(long, default_value("black"), value_name("COLOR"))]
    pub foreground: Color,

    /// Background color.
    ///
    /// <COLOR> takes a CSS color string. Colored output is only available when
    /// the output format is PNG, SVG or any ANSI escape sequences. Note that
    /// lossy conversion may be performed depending on the color space supported
    /// by the method to specify a color, the color depth supported by the
    /// output format, etc.
    #[arg(long, default_value("white"), value_name("COLOR"))]
    pub background: Color,

    /// Also print the metadata.
    ///
    /// It is output to stderr.
    #[arg(long)]
    pub verbose: bool,

    /// Input data.
    ///
    /// If [STRING] is not specified, data will be read from standard input.
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
    /// read from standard input. Supported raster image formats are based on
    /// the formats supported by the image crate. The format of [IMAGE] is
    /// determined based on the extension or the magic number if possible. If
    /// the format cannot be determined, use '--type'. Note that the SVG image
    /// is rasterized before scanning.
    #[arg(value_name("IMAGE"), value_hint(ValueHint::FilePath))]
    pub input: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct Completion {
    /// Shell to generate completion for.
    #[arg(value_enum, ignore_case(true))]
    pub shell: Shell,
}

impl Opt {
    /// Validates arguments.
    pub fn validate(self) -> anyhow::Result<Self> {
        if let Command::Encode(ref arg) = self.command {
            #[cfg(feature = "optimize-output-png")]
            if arg.optimize_png.is_some() && (arg.output_format != OutputFormat::Png) {
                return Err(anyhow!("output format is not PNG"));
            }
            let is_monochrome = matches!(
                arg.output_format,
                OutputFormat::Pic
                    | OutputFormat::Ascii
                    | OutputFormat::AsciiInvert
                    | OutputFormat::Unicode
                    | OutputFormat::UnicodeInvert
            );
            let is_default_colors = (arg.foreground
                == Color::from_rgba8(u8::MIN, u8::MIN, u8::MIN, u8::MAX))
                && (arg.background == Color::from_rgba8(u8::MAX, u8::MAX, u8::MAX, u8::MAX));
            if is_monochrome && !is_default_colors {
                return Err(anyhow!(
                    "foreground and/or background colors cannot be changed"
                ));
            }
        }
        Ok(self)
    }

    /// Generates shell completion and print it.
    pub fn print_completion(generator: impl Generator) {
        clap_complete::generate(
            generator,
            &mut Self::command(),
            Self::command().get_name(),
            &mut io::stdout(),
        );
    }
}

#[derive(Clone, Debug, ValueEnum)]
#[allow(clippy::doc_markdown)]
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

    /// PIC markup language.
    Pic,

    /// To the terminal using 4-bit ANSI escape sequences.
    #[cfg(feature = "output-as-ansi")]
    Ansi,

    /// To the terminal using 8-bit ANSI escape sequences.
    #[cfg(feature = "output-as-ansi")]
    Ansi256,

    /// To the terminal using 24-bit ANSI escape sequences.
    #[cfg(feature = "output-as-ansi")]
    AnsiTrueColor,

    /// To the terminal as ASCII string.
    Ascii,

    /// To the terminal as ASCII string.
    ///
    /// This value inverts foreground and background colors.
    #[value(alias("ASCIIi"))]
    AsciiInvert,

    /// To the terminal as UTF-8 string.
    #[value(alias("terminal"), alias("UTF8"))]
    Unicode,

    /// To the terminal as UTF-8 string.
    ///
    /// This value inverts foreground and background colors.
    #[value(alias("UTF8i"))]
    UnicodeInvert,
}

#[cfg(feature = "optimize-output-png")]
#[derive(Clone, Debug, ValueEnum)]
pub enum PngOptimizationLevel {
    /// Level 0.
    ///
    /// This value is the minimum optimization level.
    #[value(name = "0")]
    Level0,

    /// Level 1.
    #[value(name = "1")]
    Level1,

    /// Level 2.
    #[value(name = "2")]
    Level2,

    /// Level 3.
    #[value(name = "3")]
    Level3,

    /// Level 4.
    #[value(name = "4")]
    Level4,

    /// Level 5.
    #[value(name = "5")]
    Level5,

    /// Level 6.
    ///
    /// This value is the maximum optimization level.
    #[value(name = "6", alias("max"))]
    Level6,
}

#[cfg(feature = "optimize-output-png")]
impl From<PngOptimizationLevel> for u8 {
    fn from(level: PngOptimizationLevel) -> Self {
        level as Self
    }
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

#[derive(Clone, Copy, Debug, ValueEnum)]
#[allow(clippy::doc_markdown)]
#[value(rename_all = "lower")]
pub enum InputFormat {
    /// Windows Bitmap.
    #[cfg(feature = "decode-from-bmp")]
    Bmp,

    /// DirectDraw Surface.
    #[cfg(feature = "decode-from-dds")]
    Dds,

    /// Farbfeld.
    #[cfg(feature = "decode-from-ff")]
    Farbfeld,

    /// Graphics Interchange Format.
    #[cfg(feature = "decode-from-gif")]
    Gif,

    /// Radiance RGBE.
    #[cfg(feature = "decode-from-hdr")]
    Hdr,

    /// ICO file format.
    ///
    /// This value also includes the CUR file format.
    #[cfg(feature = "decode-from-ico")]
    Ico,

    /// JPEG.
    #[cfg(feature = "decode-from-jpeg")]
    Jpeg,

    /// OpenEXR.
    #[cfg(feature = "decode-from-exr")]
    OpenExr,

    /// Portable Network Graphics.
    Png,

    /// Portable Anymap Format.
    #[cfg(feature = "decode-from-pnm")]
    Pnm,

    /// Quite OK Image Format.
    #[cfg(feature = "decode-from-qoi")]
    Qoi,

    /// Scalable Vector Graphics.
    ///
    /// This value also includes the gzip-compressed SVG image.
    #[cfg(feature = "decode-from-svg")]
    Svg,

    /// Truevision TGA.
    #[cfg(feature = "decode-from-tga")]
    Tga,

    /// Tag Image File Format.
    #[cfg(feature = "decode-from-tiff")]
    Tiff,

    /// WebP.
    #[cfg(feature = "decode-from-webp")]
    WebP,

    /// X BitMap.
    #[cfg(feature = "decode-from-xbm")]
    Xbm,
}

impl TryFrom<InputFormat> for ImageFormat {
    type Error = ImageError;

    fn try_from(format: InputFormat) -> Result<Self, Self::Error> {
        match format {
            #[cfg(feature = "decode-from-bmp")]
            InputFormat::Bmp => Ok(Self::Bmp),
            #[cfg(feature = "decode-from-dds")]
            InputFormat::Dds => Ok(Self::Dds),
            #[cfg(feature = "decode-from-ff")]
            InputFormat::Farbfeld => Ok(Self::Farbfeld),
            #[cfg(feature = "decode-from-gif")]
            InputFormat::Gif => Ok(Self::Gif),
            #[cfg(feature = "decode-from-hdr")]
            InputFormat::Hdr => Ok(Self::Hdr),
            #[cfg(feature = "decode-from-ico")]
            InputFormat::Ico => Ok(Self::Ico),
            #[cfg(feature = "decode-from-jpeg")]
            InputFormat::Jpeg => Ok(Self::Jpeg),
            #[cfg(feature = "decode-from-exr")]
            InputFormat::OpenExr => Ok(Self::OpenExr),
            InputFormat::Png => Ok(Self::Png),
            #[cfg(feature = "decode-from-pnm")]
            InputFormat::Pnm => Ok(Self::Pnm),
            #[cfg(feature = "decode-from-qoi")]
            InputFormat::Qoi => Ok(Self::Qoi),
            #[cfg(feature = "decode-from-svg")]
            InputFormat::Svg => Err(Self::Error::Unsupported(
                image::error::ImageFormatHint::Unknown.into(),
            )),
            #[cfg(feature = "decode-from-tga")]
            InputFormat::Tga => Ok(Self::Tga),
            #[cfg(feature = "decode-from-tiff")]
            InputFormat::Tiff => Ok(Self::Tiff),
            #[cfg(feature = "decode-from-webp")]
            InputFormat::WebP => Ok(Self::WebP),
            #[cfg(feature = "decode-from-xbm")]
            InputFormat::Xbm => Err(Self::Error::Unsupported(
                image::error::ImageFormatHint::Unknown.into(),
            )),
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

    #[cfg(feature = "optimize-output-png")]
    #[test]
    fn from_png_optimization_level_to_u8() {
        assert_eq!(u8::from(PngOptimizationLevel::Level0), 0);
        assert_eq!(u8::from(PngOptimizationLevel::Level1), 1);
        assert_eq!(u8::from(PngOptimizationLevel::Level2), 2);
        assert_eq!(u8::from(PngOptimizationLevel::Level3), 3);
        assert_eq!(u8::from(PngOptimizationLevel::Level4), 4);
        assert_eq!(u8::from(PngOptimizationLevel::Level5), 5);
        assert_eq!(u8::from(PngOptimizationLevel::Level6), 6);
    }

    #[test]
    fn default_variant() {
        assert_eq!(Variant::default(), Variant::Normal);
    }

    #[test]
    fn try_from_input_format_to_image_format() {
        #[cfg(feature = "decode-from-bmp")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Bmp).unwrap(),
            ImageFormat::Bmp
        );
        #[cfg(feature = "decode-from-dds")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Dds).unwrap(),
            ImageFormat::Dds
        );
        #[cfg(feature = "decode-from-ff")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Farbfeld).unwrap(),
            ImageFormat::Farbfeld
        );
        #[cfg(feature = "decode-from-gif")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Gif).unwrap(),
            ImageFormat::Gif
        );
        #[cfg(feature = "decode-from-hdr")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Hdr).unwrap(),
            ImageFormat::Hdr
        );
        #[cfg(feature = "decode-from-ico")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Ico).unwrap(),
            ImageFormat::Ico
        );
        #[cfg(feature = "decode-from-jpeg")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Jpeg).unwrap(),
            ImageFormat::Jpeg
        );
        #[cfg(feature = "decode-from-exr")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::OpenExr).unwrap(),
            ImageFormat::OpenExr
        );
        assert_eq!(
            ImageFormat::try_from(InputFormat::Png).unwrap(),
            ImageFormat::Png
        );
        #[cfg(feature = "decode-from-pnm")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Pnm).unwrap(),
            ImageFormat::Pnm
        );
        #[cfg(feature = "decode-from-qoi")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Qoi).unwrap(),
            ImageFormat::Qoi
        );
        #[cfg(feature = "decode-from-svg")]
        assert!(ImageFormat::try_from(InputFormat::Svg).is_err());
        #[cfg(feature = "decode-from-tga")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Tga).unwrap(),
            ImageFormat::Tga
        );
        #[cfg(feature = "decode-from-tiff")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::Tiff).unwrap(),
            ImageFormat::Tiff
        );
        #[cfg(feature = "decode-from-webp")]
        assert_eq!(
            ImageFormat::try_from(InputFormat::WebP).unwrap(),
            ImageFormat::WebP
        );
        #[cfg(feature = "decode-from-xbm")]
        assert!(ImageFormat::try_from(InputFormat::Xbm).is_err());
    }
}
