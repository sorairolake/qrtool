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
    QrCode, QrResult, Version,
};

use crate::cli::{Mode, Variant};

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
pub fn to_svg(code: &QrCode, margin: u32) -> String {
    Renderer::<svg::Color<'_>>::new(&code.to_colors(), code.width(), margin).build()
}

/// Renders the QR code into an image.
pub fn to_unicode(code: &QrCode, margin: u32) -> String {
    Renderer::<unicode::Dense1x2>::new(&code.to_colors(), code.width(), margin).build()
}

/// Renders the QR code into an image.
pub fn to_image(code: &QrCode, margin: u32) -> DynamicImage {
    let image = Renderer::<Luma<u8>>::new(&code.to_colors(), code.width(), margin).build();
    DynamicImage::ImageLuma8(image)
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
}
