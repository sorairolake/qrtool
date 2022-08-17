//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufReader, Cursor};
use std::path::Path;

use anyhow::Context;
use image::{io::Reader, DynamicImage, ImageError, ImageFormat, ImageResult};
use rqrr::{BitGrid, DeQRError, Grid, MetaData};
use tiny_skia::{Pixmap, Transform};
use usvg::{FitTo, Tree};

/// Returns `true` if `path` is SVG.
pub fn is_svg(path: impl AsRef<Path>) -> bool {
    matches!(
        path.as_ref().extension().and_then(OsStr::to_str),
        Some("svg" | "svgz")
    )
}

fn svg_to_png(path: &Path) -> anyhow::Result<Vec<u8>> {
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

fn from_png(data: &[u8]) -> ImageResult<DynamicImage> {
    Reader::with_format(Cursor::new(data), ImageFormat::Png).decode()
}

/// Reads the image from SVG.
pub fn from_svg(path: impl AsRef<Path>) -> anyhow::Result<DynamicImage> {
    let data = svg_to_png(path.as_ref())?;
    from_png(&data).map_err(anyhow::Error::from)
}

/// Reads an image file.
pub fn load_image_file(path: impl AsRef<Path>, format: ImageFormat) -> ImageResult<DynamicImage> {
    let reader = BufReader::new(File::open(path.as_ref()).map_err(ImageError::IoError)?);
    image::load(reader, format)
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
