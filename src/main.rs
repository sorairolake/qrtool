//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022-2023 Shun Sakai
//

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]

mod cli;
mod color;
mod core;
mod decode;
mod encode;
mod metadata;

use std::{io, process::ExitCode};

use image::ImageError;
use qrencode::types::QrError;
use rqrr::DeQRError;

fn main() -> ExitCode {
    match core::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            #[allow(clippy::option_if_let_else)]
            if let Some(e) = err.downcast_ref::<io::Error>() {
                sysexits::ExitCode::try_from(e.kind()).map_or(ExitCode::FAILURE, ExitCode::from)
            } else if err.is::<QrError>() {
                sysexits::ExitCode::DataErr.into()
            } else if let Some(e) = err.downcast_ref::<DeQRError>() {
                match e {
                    DeQRError::IoError => sysexits::ExitCode::IoErr.into(),
                    _ => sysexits::ExitCode::DataErr.into(),
                }
            } else if let Some(e) = err.downcast_ref::<ImageError>() {
                match e {
                    ImageError::Limits(_) => sysexits::ExitCode::OsErr.into(),
                    ImageError::Unsupported(_) => sysexits::ExitCode::Unavailable.into(),
                    ImageError::IoError(_) => sysexits::ExitCode::IoErr.into(),
                    _ => sysexits::ExitCode::DataErr.into(),
                }
            } else {
                ExitCode::FAILURE
            }
        }
    }
}
