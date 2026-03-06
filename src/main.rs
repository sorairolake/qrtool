// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod app;
mod cli;
mod decode;
mod encode;
mod input;
mod metadata;

use std::{io, process::ExitCode};

use image::ImageError;
use qrcode2::types::QrError;
#[cfg(feature = "decode-from-svg")]
use resvg::usvg;
use rqrr::DeQRError;

fn main() -> ExitCode {
    match app::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            if let Some(e) = err.downcast_ref::<io::Error>() {
                return sysexits::ExitCode::from(e.kind()).into();
            }
            if err.is::<QrError>() {
                return sysexits::ExitCode::DataErr.into();
            }
            if let Some(e) = err.downcast_ref::<DeQRError>() {
                return if matches!(e, DeQRError::IoError) {
                    sysexits::ExitCode::IoErr.into()
                } else {
                    sysexits::ExitCode::DataErr.into()
                };
            }
            if let Some(e) = err.downcast_ref::<ImageError>() {
                return match e {
                    ImageError::Limits(_) => sysexits::ExitCode::OsErr.into(),
                    ImageError::Unsupported(_) => sysexits::ExitCode::Unavailable.into(),
                    ImageError::IoError(_) => sysexits::ExitCode::IoErr.into(),
                    _ => sysexits::ExitCode::DataErr.into(),
                };
            }
            #[cfg(feature = "decode-from-svg")]
            if let Some(e) = err.downcast_ref::<usvg::Error>() {
                return match e {
                    usvg::Error::NotAnUtf8Str | usvg::Error::ElementsLimitReached => {
                        sysexits::ExitCode::Unavailable.into()
                    }
                    usvg::Error::MalformedGZip
                    | usvg::Error::InvalidSize
                    | usvg::Error::ParsingFailed(_) => sysexits::ExitCode::DataErr.into(),
                };
            }
            ExitCode::FAILURE
        }
    }
}
