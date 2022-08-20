//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use image_for_encoding::{DynamicImage, Luma};
use qrcode::{
    bits::Bits,
    render::{svg, unicode, Renderer},
    types::QrError,
    EcLevel, QrCode, QrResult, Version,
};

use crate::cli::{Ecc, Mode, Variant};
use crate::metadata::{Extractor, Metadata};

/// Sets the version.
pub const fn set_version(version: i16, variant: &Variant) -> QrResult<Version> {
    match variant {
        Variant::Normal => {
            if let 1..=40 = version {
                Ok(Version::Normal(version))
            } else {
                Err(QrError::InvalidVersion)
            }
        }
        Variant::Micro => {
            if let 1..=4 = version {
                Ok(Version::Micro(version))
            } else {
                Err(QrError::InvalidVersion)
            }
        }
    }
}

/// Encodes data for the selected mode to the bits.
pub fn push_data_for_selected_mode(
    bits: &mut Bits,
    data: impl AsRef<[u8]>,
    mode: &Mode,
) -> QrResult<()> {
    let data = data.as_ref();
    match mode {
        Mode::Numeric => bits.push_numeric_data(data),
        Mode::Alphanumeric => bits.push_alphanumeric_data(data),
        Mode::Byte => bits.push_byte_data(data),
        Mode::Kanji => bits.push_kanji_data(data),
    }
}

/// Renders the QR code into an image.
#[cfg(not(feature = "color-output"))]
pub fn to_svg(code: &QrCode, margin: u32) -> String {
    Renderer::<svg::Color<'_>>::new(&code.to_colors(), code.width(), margin).build()
}

/// Renders the QR code into an image.
#[cfg(feature = "color-output")]
pub fn to_svg(
    code: &QrCode,
    margin: u32,
    colors: (Option<crate::util::Color>, Option<crate::util::Color>),
) -> String {
    use qrcode::{render::Pixel, Color};

    let foreground = colors.0.map_or_else(
        || svg::Color::default_color(Color::Dark).0.to_string(),
        |fg| format!("#{fg:x}"),
    );
    let background = colors.1.map_or_else(
        || svg::Color::default_color(Color::Light).0.to_string(),
        |bg| format!("#{bg:x}"),
    );
    Renderer::<svg::Color<'_>>::new(&code.to_colors(), code.width(), margin)
        .dark_color(svg::Color(&foreground))
        .light_color(svg::Color(&background))
        .build()
}

/// Renders the QR code into the terminal as UTF-8 string.
pub fn to_terminal(code: &QrCode, margin: u32) -> String {
    Renderer::<unicode::Dense1x2>::new(&code.to_colors(), code.width(), margin)
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build()
}

/// Renders the QR code into an image.
#[cfg(not(feature = "color-output"))]
pub fn to_image(code: &QrCode, margin: u32) -> DynamicImage {
    let image = Renderer::<Luma<u8>>::new(&code.to_colors(), code.width(), margin).build();
    DynamicImage::ImageLuma8(image)
}

/// Renders the QR code into an image.
#[cfg(feature = "color-output")]
pub fn to_image(
    code: &QrCode,
    margin: u32,
    colors: (Option<crate::util::Color>, Option<crate::util::Color>),
) -> DynamicImage {
    use image_for_encoding::Rgb;
    use qrcode::{render::Pixel, Color};

    let foreground = colors.0.map_or_else(
        || Rgb::default_color(Color::Dark),
        |fg| {
            let rgb = fg.into_components();
            Rgb([rgb.0, rgb.1, rgb.2])
        },
    );
    let background = colors.1.map_or_else(
        || Rgb::default_color(Color::Light),
        |bg| {
            let rgb = bg.into_components();
            Rgb([rgb.0, rgb.1, rgb.2])
        },
    );
    if foreground == Rgb::default_color(Color::Dark)
        && background == Rgb::default_color(Color::Light)
    {
        let image = Renderer::<Luma<u8>>::new(&code.to_colors(), code.width(), margin).build();
        DynamicImage::ImageLuma8(image)
    } else {
        let image = Renderer::<Rgb<u8>>::new(&code.to_colors(), code.width(), margin)
            .dark_color(foreground)
            .light_color(background)
            .build();
        DynamicImage::ImageRgb8(image)
    }
}

impl Extractor for QrCode {
    fn extract_metadata(&self) -> Metadata {
        let symbol_version = match self.version() {
            Version::Normal(version) | Version::Micro(version) => {
                usize::try_from(version).expect("Invalid symbol version")
            }
        };
        let error_correction_level = match self.error_correction_level() {
            EcLevel::L => Ecc::L,
            EcLevel::M => Ecc::M,
            EcLevel::Q => Ecc::Q,
            EcLevel::H => Ecc::H,
        };
        Metadata {
            symbol_version,
            error_correction_level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_qr_code_version() {
        // Valid normal QR code version.
        assert_eq!(
            set_version(1, &Variant::Normal).unwrap(),
            Version::Normal(1)
        );
        assert_eq!(
            set_version(40, &Variant::Normal).unwrap(),
            Version::Normal(40)
        );

        // Valid Micro QR code version.
        assert_eq!(set_version(1, &Variant::Micro).unwrap(), Version::Micro(1));
        assert_eq!(set_version(4, &Variant::Micro).unwrap(), Version::Micro(4));

        // Invalid normal QR code version.
        assert!(set_version(0, &Variant::Normal).is_err());
        assert!(set_version(41, &Variant::Normal).is_err());

        // Invalid Micro QR code version.
        assert!(set_version(0, &Variant::Micro).is_err());
        assert!(set_version(5, &Variant::Micro).is_err());
    }

    #[test]
    fn validate_metadata_extraction() {
        const DATA: [u8; 0] = [];

        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::L)
                .unwrap()
                .extract_metadata(),
            Metadata {
                symbol_version: 1,
                error_correction_level: Ecc::L
            }
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::M)
                .unwrap()
                .extract_metadata(),
            Metadata {
                symbol_version: 1,
                error_correction_level: Ecc::M
            }
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::Q)
                .unwrap()
                .extract_metadata(),
            Metadata {
                symbol_version: 1,
                error_correction_level: Ecc::Q
            }
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::H)
                .unwrap()
                .extract_metadata(),
            Metadata {
                symbol_version: 1,
                error_correction_level: Ecc::H
            }
        );
    }
}
