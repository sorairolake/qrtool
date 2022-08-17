//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::io;
use std::path::PathBuf;

use clap::{value_parser, AppSettings, CommandFactory, Parser, Subcommand, ValueEnum, ValueHint};
use clap_complete::{Generator, Shell};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
#[clap(version, about, propagate_version(true), arg_required_else_help(true))]
#[clap(setting(AppSettings::DeriveDisplayOrder))]
pub struct Opt {
    /// Generate shell completion.
    ///
    /// The completion is output to stdout.
    #[clap(long, value_enum, value_name("SHELL"))]
    pub(crate) generate_completion: Option<Shell>,

    #[clap(subcommand)]
    pub(crate) command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Encode input data in a QR code.
    #[clap(setting(AppSettings::DeriveDisplayOrder))]
    Encode {
        /// Output the result to a file.
        #[clap(short, long, value_name("FILE"))]
        output: Option<PathBuf>,

        /// Read input data from a file.
        #[clap(
            short,
            long,
            value_name("FILE"),
            value_hint(ValueHint::FilePath),
            conflicts_with("input")
        )]
        read_from: Option<PathBuf>,

        /// Error correction level.
        #[clap(short('l'), long, value_enum, default_value_t, value_name("LEVEL"))]
        error_correction_level: Ecc,

        /// The version of the symbol.
        ///
        /// For normal QR code, it should be between 1 and 40.
        /// For Micro QR code, it should be between 1 and 4.
        #[clap(
            value_parser(value_parser!(i16).range(1..=40)),
            short('v'),
            value_name("NUMBER")
        )]
        symbol_version: Option<i16>,

        /// The width of margin.
        #[clap(short, long, default_value("4"), value_name("NUMBER"))]
        margin: u32,

        /// The format of the output.
        #[clap(
            short('t'),
            long("type"),
            value_enum,
            default_value_t,
            value_name("FORMAT")
        )]
        output_format: OutputFormat,

        /// The mode of the output.
        #[clap(short('M'), long, value_enum, default_value_t, value_name("MODE"))]
        mode: Mode,

        /// The type of QR code.
        #[clap(
            long,
            value_enum,
            default_value_t,
            requires("symbol-version"),
            value_name("TYPE")
        )]
        variant: Variant,

        /// Input data.
        ///
        /// If it is not specified, data will be read from stdin.
        #[clap(value_name("STRING"))]
        input: Option<String>,
    },

    /// Detect and decode a QR code.
    #[clap(setting(AppSettings::DeriveDisplayOrder))]
    Decode {
        /// The format of the input.
        #[clap(short('t'), long("type"), value_enum, value_name("FORMAT"))]
        input_format: Option<InputFormat>,

        /// Input image file.
        #[clap(value_name("IMAGE"), value_hint(ValueHint::FilePath))]
        input: PathBuf,
    },
}

impl Opt {
    /// Generate shell completion and print it.
    pub(crate) fn print_completion(gen: impl Generator) {
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

impl TryFrom<OutputFormat> for image::ImageFormat {
    type Error = image::ImageError;

    fn try_from(format: OutputFormat) -> Result<Self, Self::Error> {
        match format {
            OutputFormat::Png => Ok(Self::Png),
            _ => Err(image::ImageError::Unsupported(
                image::error::ImageFormatHint::Unknown.into(),
            )),
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
    Svg,
}

impl TryFrom<InputFormat> for image::ImageFormat {
    type Error = image::ImageError;

    fn try_from(format: InputFormat) -> Result<Self, Self::Error> {
        match format {
            InputFormat::Png => Ok(Self::Png),
            InputFormat::Svg => Err(image::ImageError::Unsupported(
                image::error::ImageFormatHint::Unknown.into(),
            )),
        }
    }
}
