//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use image::{DynamicImage, Luma};
use qrcode::{
    bits::Bits,
    render::{svg, unicode, Renderer},
    QrCode, QrResult, Version,
};

use crate::cli::{Mode, Variant};

/// Sets the version.
pub const fn set_version(version: i16, variant: &Variant) -> Version {
    match variant {
        Variant::Normal => Version::Normal(version),
        Variant::Micro => Version::Micro(version),
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
