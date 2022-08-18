//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::io;
use std::path::PathBuf;

use clap::{
    value_parser, AppSettings, Args, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint,
};
use clap_complete::{Generator, Shell};
use image::{error::ImageFormatHint, ImageError, ImageFormat};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[clap(
    version,
    about,
    propagate_version(true),
    arg_required_else_help(true),
    args_conflicts_with_subcommands(true)
)]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
pub struct Opt {
    /// Generate shell completion.
    ///
    /// The completion is output to stdout.
    #[clap(long, value_enum, value_name("SHELL"))]
    pub generate_completion: Option<Shell>,

    #[clap(subcommand)]
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
#[clap(setting(AppSettings::DeriveDisplayOrder))]
pub struct Encode {
    /// Output the result to a file.
    #[clap(short, long, value_name("FILE"))]
    pub output: Option<PathBuf>,

    /// Read input data from a file.
    #[clap(
        short,
        long,
        value_name("FILE"),
        value_hint(ValueHint::FilePath),
        conflicts_with("input")
    )]
    pub read_from: Option<PathBuf>,

    /// Error correction level.
    #[clap(short('l'), long, value_enum, default_value_t, value_name("LEVEL"))]
    pub error_correction_level: Ecc,

    /// The version of the symbol.
    ///
    /// For normal QR code, it should be between 1 and 40.
    /// For Micro QR code, it should be between 1 and 4.
    #[clap(
        value_parser(value_parser!(i16).range(1..=40)),
        short('v'),
        long,
        value_name("NUMBER")
    )]
    pub symbol_version: Option<i16>,

    /// The width of margin.
    #[clap(short, long, default_value("4"), value_name("NUMBER"))]
    pub margin: u32,

    /// The format of the output.
    #[clap(
        short('t'),
        long("type"),
        value_enum,
        default_value_t,
        value_name("FORMAT")
    )]
    pub output_format: OutputFormat,

    /// The mode of the output.
    #[clap(short('M'), long, value_enum, default_value_t, value_name("MODE"))]
    pub mode: Mode,

    /// The type of QR code.
    #[clap(
        long,
        value_enum,
        default_value_t,
        requires("symbol-version"),
        value_name("TYPE")
    )]
    pub variant: Variant,

    /// Input data.
    ///
    /// If it is not specified, data will be read from stdin.
    #[clap(value_name("STRING"))]
    pub input: Option<String>,
}

#[derive(Args, Debug)]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
pub struct Decode {
    /// The format of the input.
    #[clap(short('t'), long("type"), value_enum, value_name("FORMAT"))]
    pub input_format: Option<InputFormat>,

    /// Input image file.
    #[clap(value_name("IMAGE"), value_hint(ValueHint::FilePath))]
    pub input: PathBuf,
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

#[derive(Clone, Debug, ValueEnum)]
pub enum Ecc {
    /// Level L.
    ///
    /// 7% of codewords can be restored.
    L,

    /// Level M.
    ///
    /// 15% of codewords can be restored.
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

impl Default for Ecc {
    fn default() -> Self {
        Self::M
    }
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

#[derive(Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    /// Portable Network Graphics.
    Png,

    /// Scalable Vector Graphics.
    Svg,

    /// UTF-8 string.
    Unicode,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Png
    }
}

impl TryFrom<OutputFormat> for ImageFormat {
    type Error = ImageError;

    fn try_from(format: OutputFormat) -> Result<Self, Self::Error> {
        match format {
            OutputFormat::Png => Ok(Self::Png),
            _ => Err(ImageError::Unsupported(ImageFormatHint::Unknown.into())),
        }
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Mode {
    /// Numbers from 0 to 9.
    Numeric,

    /// Uppercase letters from A to Z, numbers from 0 to 9 and few symbols.
    Alphanumeric,

    /// Arbitrary binary data.
    Byte,

    /// Shift JIS text.
    Kanji,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Byte
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Variant {
    /// Normal QR code.
    Normal,

    /// Micro QR code.
    Micro,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Debug, ValueEnum)]
pub enum InputFormat {
    /// Portable Network Graphics.
    Png,

    /// Scalable Vector Graphics.
    ///
    /// This also includes gzipped it.
    Svg,
}

impl TryFrom<InputFormat> for ImageFormat {
    type Error = ImageError;

    fn try_from(format: InputFormat) -> Result<Self, Self::Error> {
        match format {
            InputFormat::Png => Ok(Self::Png),
            InputFormat::Svg => Err(ImageError::Unsupported(ImageFormatHint::Unknown.into())),
        }
    }
}
