// SPDX-FileCopyrightText: 2022 Shun Sakai
// SPDX-FileCopyrightText: 2024 Alexis Hildebrandt
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "output-as-ansi")]
use anstyle::RgbColor;
#[cfg(feature = "output-as-ansi")]
use anstyle_lossy::palette::Palette;
use csscolorparser::Color;
use image::{Rgba, RgbaImage};
use qrcode2::{
    QrCode, QrResult, Version,
    bits::Bits,
    render::{Renderer, pic, svg, unicode::Dense1x2},
    types::QrError,
};
#[cfg(feature = "output-as-ansi")]
use yansi::Paint;

use crate::{
    cli::{Mode, Variant},
    metadata::{self, Extractor, Metadata},
};

/// Sets the version.
pub fn set_version(version: &[i16], variant: &Variant) -> QrResult<Version> {
    match variant {
        Variant::Normal => Some(Version::Normal(version[0]))
            .filter(|v| v.is_normal())
            .ok_or(QrError::InvalidVersion),
        Variant::Micro => Some(Version::Micro(version[0]))
            .filter(|v| v.is_micro())
            .ok_or(QrError::InvalidVersion),
        Variant::Rmqr => Some(Version::RectMicro(
            version[0],
            version.get(1).copied().unwrap_or_default(),
        ))
        .filter(|v| v.is_rect_micro())
        .ok_or(QrError::InvalidVersion),
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
pub fn to_image(
    code: &QrCode,
    margin: u32,
    colors: &(Color, Color),
    module_size: Option<u32>,
) -> RgbaImage {
    let c = code.to_colors();
    let mut renderer = &mut Renderer::<Rgba<u8>>::new(&c, code.width(), code.height(), margin);
    renderer = renderer
        .dark_color(Rgba::from(colors.0.to_rgba8()))
        .light_color(Rgba::from(colors.1.to_rgba8()));
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build()
}

/// Renders the QR code into an image.
pub fn to_svg(
    code: &QrCode,
    margin: u32,
    colors: &(Color, Color),
    module_size: Option<u32>,
) -> String {
    let c = code.to_colors();
    let mut renderer =
        &mut Renderer::<svg::Color<'_>>::new(&c, code.width(), code.height(), margin);
    let (foreground, background) = (colors.0.to_css_hex(), colors.1.to_css_hex());
    renderer = renderer
        .dark_color(svg::Color(&foreground))
        .light_color(svg::Color(&background));
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build() + "\n"
}

/// Renders the QR code into a PIC image.
pub fn to_pic(code: &QrCode, margin: u32, module_size: Option<u32>) -> String {
    let c = code.to_colors();
    let mut renderer = &mut Renderer::<pic::Color>::new(&c, code.width(), code.height(), margin);
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build() + "\n"
}

/// Renders the QR code into the terminal using 4-bit ANSI escape sequences.
#[cfg(feature = "output-as-ansi")]
pub fn to_ansi(
    code: &QrCode,
    margin: u32,
    colors: &(Color, Color),
    module_size: Option<u32>,
) -> String {
    fn convert(color: &Color) -> String {
        let rgba = color.to_rgba8();
        let ansi =
            anstyle_lossy::rgb_to_ansi(RgbColor(rgba[0], rgba[1], rgba[2]), Palette::default());
        let ansi = anstyle_yansi::to_yansi_color(ansi.into());
        debug_assert!(!matches!(
            ansi,
            yansi::Color::Primary | yansi::Color::Fixed(_) | yansi::Color::Rgb(..)
        ));
        format!("{}", "  ".bg(ansi))
    }

    let c = code.to_colors();
    let mut renderer = &mut Renderer::<&str>::new(&c, code.width(), code.height(), margin);
    let (foreground, background) = (convert(&colors.0), convert(&colors.1));
    renderer = renderer.dark_color(&foreground).light_color(&background);
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build() + "\n"
}

/// Renders the QR code into the terminal using 8-bit ANSI escape sequences.
#[cfg(feature = "output-as-ansi")]
pub fn to_ansi_256(
    code: &QrCode,
    margin: u32,
    colors: &(Color, Color),
    module_size: Option<u32>,
) -> String {
    fn convert(color: &Color) -> String {
        let rgba = color.to_rgba8();
        let ansi_256 = anstyle_lossy::rgb_to_xterm(RgbColor(rgba[0], rgba[1], rgba[2]));
        format!("{}", "  ".on_fixed(ansi_256.index()))
    }

    let c = code.to_colors();
    let mut renderer = &mut Renderer::<&str>::new(&c, code.width(), code.height(), margin);
    let (foreground, background) = (convert(&colors.0), convert(&colors.1));
    renderer = renderer.dark_color(&foreground).light_color(&background);
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build() + "\n"
}

/// Renders the QR code into the terminal using 24-bit ANSI escape sequences.
#[cfg(feature = "output-as-ansi")]
pub fn to_ansi_true_color(
    code: &QrCode,
    margin: u32,
    colors: &(Color, Color),
    module_size: Option<u32>,
) -> String {
    let c = code.to_colors();
    let mut renderer = &mut Renderer::<&str>::new(&c, code.width(), code.height(), margin);
    let (foreground, background) = (
        {
            let fg = colors.0.to_rgba8();
            format!("{}", "  ".on_rgb(fg[0], fg[1], fg[2]))
        },
        {
            let bg = colors.1.to_rgba8();
            format!("{}", "  ".on_rgb(bg[0], bg[1], bg[2]))
        },
    );
    renderer = renderer.dark_color(&foreground).light_color(&background);
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build() + "\n"
}

/// Renders the QR code into the terminal as ASCII string.
pub fn to_ascii(code: &QrCode, margin: u32, module_size: Option<u32>, invert: bool) -> String {
    let c = code.to_colors();
    let mut renderer = &mut Renderer::<&str>::new(&c, code.width(), code.height(), margin);
    renderer = if invert {
        renderer.dark_color("  ").light_color("##")
    } else {
        renderer.dark_color("##").light_color("  ")
    };
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build() + "\n"
}

/// Renders the QR code into the terminal as UTF-8 string.
pub fn to_unicode(code: &QrCode, margin: u32, module_size: Option<u32>, invert: bool) -> String {
    let c = code.to_colors();
    let mut renderer = &mut Renderer::<Dense1x2>::new(&c, code.width(), code.height(), margin);
    if !invert {
        renderer = renderer
            .dark_color(Dense1x2::Light)
            .light_color(Dense1x2::Dark);
    }
    if let Some(size) = module_size {
        renderer = renderer.module_dimensions(size, size);
    }
    renderer.build() + "\n"
}

impl Extractor for QrCode {
    fn metadata(&self) -> Metadata {
        let symbol_version = match self.version() {
            Version::Normal(version) | Version::Micro(version) => (
                usize::try_from(version).expect("invalid symbol version"),
                None,
            ),
            Version::RectMicro(height, width) => (
                usize::try_from(height).expect("invalid symbol version"),
                Some(usize::try_from(width).expect("invalid symbol version")),
            ),
        };
        let symbol_version = metadata::Version::new(symbol_version);
        let error_correction_level = self.error_correction_level().into();
        Metadata::new(symbol_version, error_correction_level)
    }
}

#[cfg(test)]
mod tests {
    use qrcode2::EcLevel;

    use super::*;
    use crate::cli::Ecc;

    #[test]
    fn validate_qr_code_version() {
        // Valid normal QR code version.
        assert_eq!(
            set_version(&[1], &Variant::Normal).unwrap(),
            Version::Normal(1)
        );
        assert_eq!(
            set_version(&[40], &Variant::Normal).unwrap(),
            Version::Normal(40)
        );

        // Valid Micro QR code version.
        assert_eq!(
            set_version(&[1], &Variant::Micro).unwrap(),
            Version::Micro(1)
        );
        assert_eq!(
            set_version(&[4], &Variant::Micro).unwrap(),
            Version::Micro(4)
        );

        // Valid rMQR code version.
        assert_eq!(
            set_version(&[7, 43], &Variant::Rmqr).unwrap(),
            Version::RectMicro(7, 43)
        );
        assert_eq!(
            set_version(&[11, 27], &Variant::Rmqr).unwrap(),
            Version::RectMicro(11, 27)
        );
        assert_eq!(
            set_version(&[17, 139], &Variant::Rmqr).unwrap(),
            Version::RectMicro(17, 139)
        );

        // Invalid normal QR code version.
        assert!(set_version(&[0], &Variant::Normal).is_err());
        assert!(set_version(&[41], &Variant::Normal).is_err());

        // Invalid Micro QR code version.
        assert!(set_version(&[0], &Variant::Micro).is_err());
        assert!(set_version(&[5], &Variant::Micro).is_err());

        // Invalid rMQR code version.
        assert!(set_version(&[0, 0], &Variant::Rmqr).is_err());
        assert!(set_version(&[7], &Variant::Rmqr).is_err());
    }

    #[test]
    fn validate_metadata_extraction() {
        const DATA: [u8; 0] = [];

        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::L)
                .unwrap()
                .metadata(),
            Metadata::new(metadata::Version::new((1, None)), Ecc::L)
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::M)
                .unwrap()
                .metadata(),
            Metadata::new(metadata::Version::new((1, None)), Ecc::M)
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::Q)
                .unwrap()
                .metadata(),
            Metadata::new(metadata::Version::new((1, None)), Ecc::Q)
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Normal(1), EcLevel::H)
                .unwrap()
                .metadata(),
            Metadata::new(metadata::Version::new((1, None)), Ecc::H)
        );

        assert_eq!(
            QrCode::with_version(DATA, Version::Micro(4), EcLevel::L)
                .unwrap()
                .metadata(),
            Metadata::new(metadata::Version::new((4, None)), Ecc::L)
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Micro(4), EcLevel::M)
                .unwrap()
                .metadata(),
            Metadata::new(metadata::Version::new((4, None)), Ecc::M)
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::Micro(4), EcLevel::Q)
                .unwrap()
                .metadata(),
            Metadata::new(metadata::Version::new((4, None)), Ecc::Q)
        );

        assert_eq!(
            QrCode::with_version(DATA, Version::RectMicro(7, 43), EcLevel::M)
                .unwrap()
                .metadata(),
            Metadata::new(metadata::Version::new((7, Some(43))), Ecc::M)
        );
        assert_eq!(
            QrCode::with_version(DATA, Version::RectMicro(7, 43), EcLevel::H)
                .unwrap()
                .metadata(),
            Metadata::new(metadata::Version::new((7, Some(43))), Ecc::H)
        );
    }
}
