// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use rqrr::{BitGrid, DeQRError, Grid, MetaData};

use crate::{
    cli::Ecc,
    metadata::{Extractor, Metadata},
};

/// Returns `true` if `path` is SVG.
#[cfg(feature = "decode-from-svg")]
pub fn is_svg(path: impl AsRef<std::path::Path>) -> bool {
    use std::ffi::OsStr;

    matches!(
        path.as_ref().extension().and_then(OsStr::to_str),
        Some("svg" | "svgz")
    )
}

#[cfg(feature = "decode-from-svg")]
fn svg_to_png(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    use anyhow::Context;
    use resvg::{
        tiny_skia::{Pixmap, Transform},
        usvg,
    };

    let opt = usvg::Options::default();

    let tree = usvg::Tree::from_data(data, &opt)?;

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .context("could not allocate a new pixmap")?;
    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());
    pixmap.encode_png().map_err(anyhow::Error::from)
}

/// Reads the image from SVG.
#[cfg(feature = "decode-from-svg")]
pub fn from_svg(data: impl AsRef<[u8]>) -> anyhow::Result<image::DynamicImage> {
    let image = svg_to_png(data.as_ref())?;
    image::load_from_memory_with_format(&image, image::ImageFormat::Png)
        .map_err(anyhow::Error::from)
}

type DecodedBytes = (MetaData, Vec<u8>);

fn grid_as_bytes<G: BitGrid>(grid: &Grid<G>) -> Result<DecodedBytes, DeQRError> {
    let mut writer = Vec::new();
    grid.decode_to(&mut writer).map(|meta| (meta, writer))
}

/// Decodes the grids as bytes.
pub fn grids_as_bytes<G: BitGrid>(
    grids: impl AsRef<[Grid<G>]>,
) -> Result<Vec<DecodedBytes>, DeQRError> {
    grids
        .as_ref()
        .iter()
        .map(|grid| grid_as_bytes(grid))
        .collect()
}

impl Extractor for MetaData {
    fn metadata(&self) -> Metadata {
        let symbol_version = self.version.0;
        let error_correction_level = match self.ecc_level {
            0 => Ecc::M,
            1 => Ecc::L,
            2 => Ecc::H,
            3 => Ecc::Q,
            _ => panic!("invalid error correction level"),
        };
        Metadata::new(symbol_version, error_correction_level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "decode-from-svg")]
    fn valid_extension_as_svg() {
        assert!(is_svg("image.svg"));
        assert!(is_svg("image.svgz"));
    }

    #[test]
    #[cfg(feature = "decode-from-svg")]
    fn invalid_extension_as_svg() {
        assert!(!is_svg("image.png"));
    }

    #[test]
    fn validate_metadata_extraction() {
        use rqrr::Version;

        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 1,
                mask: 4
            }
            .metadata(),
            Metadata::new(1, Ecc::L)
        );
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 0,
                mask: 3
            }
            .metadata(),
            Metadata::new(1, Ecc::M)
        );
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 3,
                mask: 7
            }
            .metadata(),
            Metadata::new(1, Ecc::Q)
        );
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 2,
                mask: 4
            }
            .metadata(),
            Metadata::new(1, Ecc::H)
        );
    }
}
