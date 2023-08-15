// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::cli::Ecc;

/// Metadata for a QR code.
#[derive(Debug, Eq, PartialEq)]
pub struct Metadata {
    symbol_version: usize,
    error_correction_level: Ecc,
}

pub trait Extractor {
    /// Extracts the metadata.
    fn metadata(&self) -> Metadata;
}

impl Metadata {
    /// Constructs a new metadata.
    pub const fn new(symbol_version: usize, error_correction_level: Ecc) -> Self {
        Self {
            symbol_version,
            error_correction_level,
        }
    }

    /// Gets the symbol version.
    pub const fn symbol_version(&self) -> usize {
        self.symbol_version
    }

    /// Gets the error correction level.
    pub const fn error_correction_level(&self) -> Ecc {
        self.error_correction_level
    }
}
