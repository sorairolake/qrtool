// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "decode-from-svg")]
use anyhow::Context;
#[cfg(feature = "decode-from-svg")]
use image::{DynamicImage, RgbaImage};
#[cfg(feature = "decode-from-svg")]
use resvg::{
    tiny_skia::{Pixmap, Transform},
    usvg::{Options, Tree},
};
use rqrr::{BitGrid, DeQRError, Grid, MetaData};

use crate::metadata::{self, Extractor, Metadata};

/// Errors specific to the decode command.
#[derive(Debug)]
pub enum DecodeError {
    /// The image was read successfully but no QR code was found.
    NoQrCode,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoQrCode => write!(f, "no QR code found in the image"),
        }
    }
}

impl std::error::Error for DecodeError {}

type DecodedBytes = (MetaData, Vec<u8>);

/// Reads the image from SVG.
#[cfg(feature = "decode-from-svg")]
pub fn from_svg(data: impl AsRef<[u8]>) -> anyhow::Result<DynamicImage> {
    let opt = Options::default();
    let tree = Tree::from_data(data.as_ref(), &opt)?;

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .context("could not allocate a new pixmap")?;

    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());

    let width = pixmap.width();
    let height = pixmap.height();
    let pixels = pixmap.take_demultiplied();

    RgbaImage::from_raw(width, height, pixels)
        .context("could not construct image buffer")
        .map(DynamicImage::from)
}

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
        let symbol_version = metadata::Version::new((self.version.0, None));
        let error_correction_level = self.ecc_level.into();
        Metadata::new(symbol_version, error_correction_level)
    }
}

#[cfg(test)]
mod tests {
    use rqrr::Version;

    use super::*;
    use crate::cli::Ecc;

    #[test]
    fn validate_metadata_extraction() {
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 1,
                mask: 4
            }
            .metadata(),
            Metadata::new(metadata::Version::new((1, None)), Ecc::L)
        );
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 0,
                mask: 3
            }
            .metadata(),
            Metadata::new(metadata::Version::new((1, None)), Ecc::M)
        );
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 3,
                mask: 7
            }
            .metadata(),
            Metadata::new(metadata::Version::new((1, None)), Ecc::Q)
        );
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 2,
                mask: 4
            }
            .metadata(),
            Metadata::new(metadata::Version::new((1, None)), Ecc::H)
        );
    }
}
