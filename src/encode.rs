// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use csscolorparser::Color;
use image::{Rgba, RgbaImage};
use qrcode::{
    bits::Bits,
    render::{svg, unicode, Renderer},
    types::QrError,
    EcLevel, QrCode, QrResult, Version,
};

use crate::{
    cli::{Ecc, Mode, Variant},
    metadata::{Extractor, Metadata},
};

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
pub fn to_svg(
    code: &QrCode,
    margin: u32,
    colors: &(Color, Color),
    module_size: Option<u32>,
) -> String {
    let c = code.to_colors();
    let mut renderer = Renderer::<svg::Color<'_>>::new(&c, code.width(), margin);
    let (foreground, background) = (colors.0.to_hex_string(), colors.1.to_hex_string());
    let mut renderer = renderer
        .dark_color(svg::Color(&foreground))
        .light_color(svg::Color(&background));
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build()
}

/// Renders the QR code into the terminal as UTF-8 string.
pub fn to_terminal(code: &QrCode, margin: u32, module_size: Option<u32>) -> String {
    let c = code.to_colors();
    let mut renderer = Renderer::<unicode::Dense1x2>::new(&c, code.width(), margin);
    let mut renderer = renderer
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark);
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build()
}

/// Renders the QR code into an image.
pub fn to_image(
    code: &QrCode,
    margin: u32,
    colors: &(Color, Color),
    module_size: Option<u32>,
) -> RgbaImage {
    let c = code.to_colors();
    let mut renderer = Renderer::<Rgba<u8>>::new(&c, code.width(), margin);
    let mut renderer = renderer
        .dark_color(Rgba::from(colors.0.to_rgba8()))
        .light_color(Rgba::from(colors.1.to_rgba8()));
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build()
}

impl Extractor for QrCode {
    fn metadata(&self) -> Metadata {
        let symbol_version = match self.version() {
            Version::Normal(version) | Version::Micro(version) => {
                usize::try_from(version).expect("invalid symbol version")
            }
        };
        let error_correction_level = match self.error_correction_level() {
            EcLevel::L => Ecc::L,
            EcLevel::M => Ecc::M,
            EcLevel::Q => Ecc::Q,
            EcLevel::H => Ecc::H,
        };
        Metadata::new(symbol_version, error_correction_level)
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
                .metadata(),
            Metadata::new(1, Ecc::L)
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::M)
                .unwrap()
                .metadata(),
            Metadata::new(1, Ecc::M)
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::Q)
                .unwrap()
                .metadata(),
            Metadata::new(1, Ecc::Q)
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::H)
                .unwrap()
                .metadata(),
            Metadata::new(1, Ecc::H)
        );
    }
}
