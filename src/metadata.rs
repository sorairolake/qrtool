// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::fmt;

use crate::cli::Ecc;

/// Version for a QR code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Version(usize, Option<usize>);

impl Version {
    /// Constructs a new version.
    pub const fn new(version: (usize, Option<usize>)) -> Self {
        Self(version.0, version.1)
    }

    /// Gets the version.
    const fn to_raw(self) -> (usize, Option<usize>) {
        (self.0, self.1)
    }
}

impl fmt::Display for Version {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = self.to_raw();
        if let Some(width) = version.1 {
            write!(f, "R{}x{width}", version.0)
        } else {
            version.0.fmt(f)
        }
    }
}

/// Metadata for a QR code.
#[derive(Debug, Eq, PartialEq)]
pub struct Metadata {
    symbol_version: Version,
    error_correction_level: Ecc,
}

pub trait Extractor {
    /// Extracts the metadata.
    fn metadata(&self) -> Metadata;
}

impl Metadata {
    /// Constructs a new metadata.
    pub const fn new(symbol_version: Version, error_correction_level: Ecc) -> Self {
        Self {
            symbol_version,
            error_correction_level,
        }
    }

    /// Gets the symbol version.
    pub const fn symbol_version(&self) -> Version {
        self.symbol_version
    }

    /// Gets the error correction level.
    pub const fn error_correction_level(&self) -> Ecc {
        self.error_correction_level
    }
}
