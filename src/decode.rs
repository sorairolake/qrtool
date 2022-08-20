//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image_for_decoding::{DynamicImage, ImageError, ImageFormat, ImageResult};
use rqrr::{BitGrid, DeQRError, Grid, MetaData};

use crate::cli::Ecc;
use crate::metadata::{Extractor, Metadata};

/// Returns `true` if `path` is SVG.
#[cfg(feature = "decode-from-svg")]
pub fn is_svg(path: impl AsRef<Path>) -> bool {
    use std::ffi::OsStr;

    matches!(
        path.as_ref().extension().and_then(OsStr::to_str),
        Some("svg" | "svgz")
    )
}

#[cfg(feature = "decode-from-svg")]
fn svg_to_png(path: &Path) -> anyhow::Result<Vec<u8>> {
    use std::fs;

    use anyhow::Context;
    use tiny_skia::{Pixmap, Transform};
    use usvg::{FitTo, Tree};

    let opt = usvg::Options {
        resources_dir: path
            .canonicalize()
            .ok()
            .and_then(|p| p.parent().map(Path::to_path_buf)),
        ..Default::default()
    };

    let image = fs::read(path)?;
    let tree = Tree::from_data(&image, &opt.to_ref())?;

    let pixmap_size = tree.svg_node().size.to_screen_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .context("Could not allocate a new pixmap")?;
    resvg::render(
        &tree,
        FitTo::Original,
        Transform::default(),
        pixmap.as_mut(),
    )
    .context("SVG has an invalid size")?;
    pixmap.encode_png().map_err(anyhow::Error::from)
}

#[cfg(feature = "decode-from-svg")]
fn from_png(data: &[u8]) -> ImageResult<DynamicImage> {
    use std::io::Cursor;

    use image_for_decoding::io::Reader;

    Reader::with_format(Cursor::new(data), ImageFormat::Png).decode()
}

/// Reads the image from SVG.
#[cfg(feature = "decode-from-svg")]
pub fn from_svg(path: impl AsRef<Path>) -> anyhow::Result<DynamicImage> {
    let data = svg_to_png(path.as_ref())?;
    from_png(&data).map_err(anyhow::Error::from)
}

/// Reads an image file.
pub fn load_image_file(path: impl AsRef<Path>, format: ImageFormat) -> ImageResult<DynamicImage> {
    let reader = BufReader::new(File::open(path.as_ref()).map_err(ImageError::IoError)?);
    image_for_decoding::load(reader, format)
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
    fn extract_metadata(&self) -> Metadata {
        let symbol_version = self.version.0;
        let error_correction_level = match self.ecc_level {
            0 => Ecc::M,
            1 => Ecc::L,
            2 => Ecc::H,
            3 => Ecc::Q,
            _ => panic!("Invalid error correction level"),
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
            .extract_metadata(),
            Metadata {
                symbol_version: 1,
                error_correction_level: Ecc::L
            }
        );
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 0,
                mask: 3
            }
            .extract_metadata(),
            Metadata {
                symbol_version: 1,
                error_correction_level: Ecc::M
            }
        );
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 3,
                mask: 7
            }
            .extract_metadata(),
            Metadata {
                symbol_version: 1,
                error_correction_level: Ecc::Q
            }
        );
        assert_eq!(
            MetaData {
                version: Version(1),
                ecc_level: 2,
                mask: 4
            }
            .extract_metadata(),
            Metadata {
                symbol_version: 1,
                error_correction_level: Ecc::H
            }
        );
    }
}
