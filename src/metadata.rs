//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use crate::cli::Ecc;

#[derive(Debug, Eq, PartialEq)]
pub struct Metadata {
    pub symbol_version: usize,
    pub error_correction_level: Ecc,
}

pub trait Extractor {
    /// Extracts the metadata.
    fn extract_metadata(&self) -> Metadata;
}
