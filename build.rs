// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::{
    env, io,
    process::{Command, ExitStatus},
};

fn generate_man_page(out_dir: &str) -> io::Result<ExitStatus> {
    let man_dir = env::current_dir()?.join("docs/man/man1");
    let mut command = Command::new("asciidoctor");
    command
        .args(["-b", "manpage"])
        .args(["-a", concat!("revnumber=", env!("CARGO_PKG_VERSION"))]);
    #[cfg(feature = "decode-from-bmp")]
    command.args(["-a", "decode-from-bmp"]);
    #[cfg(feature = "decode-from-dds")]
    command.args(["-a", "decode-from-dds"]);
    #[cfg(feature = "decode-from-ff")]
    command.args(["-a", "decode-from-ff"]);
    #[cfg(feature = "decode-from-gif")]
    command.args(["-a", "decode-from-gif"]);
    #[cfg(feature = "decode-from-hdr")]
    command.args(["-a", "decode-from-hdr"]);
    #[cfg(feature = "decode-from-ico")]
    command.args(["-a", "decode-from-ico"]);
    #[cfg(feature = "decode-from-jpeg")]
    command.args(["-a", "decode-from-jpeg"]);
    #[cfg(feature = "decode-from-exr")]
    command.args(["-a", "decode-from-exr"]);
    #[cfg(feature = "decode-from-pnm")]
    command.args(["-a", "decode-from-pnm"]);
    #[cfg(feature = "decode-from-qoi")]
    command.args(["-a", "decode-from-qoi"]);
    #[cfg(feature = "decode-from-tga")]
    command.args(["-a", "decode-from-tga"]);
    #[cfg(feature = "decode-from-tiff")]
    command.args(["-a", "decode-from-tiff"]);
    #[cfg(feature = "decode-from-webp")]
    command.args(["-a", "decode-from-webp"]);
    #[cfg(feature = "decode-from-svg")]
    command.args(["-a", "decode-from-svg"]);
    #[cfg(feature = "optimize-output-png")]
    command.args(["-a", "optimize-output-png"]);
    #[cfg(feature = "output-as-ansi")]
    command.args(["-a", "output-as-ansi"]);
    command
        .args(["-D", out_dir])
        .arg(man_dir.join("*.1.adoc"))
        .status()
}

fn main() {
    println!("cargo:rerun-if-changed=docs/man");

    let out_dir = env::var("OUT_DIR").expect("environment variable `OUT_DIR` not defined");
    match generate_man_page(&out_dir) {
        Ok(exit_status) => {
            if !exit_status.success() {
                println!("cargo:warning=Asciidoctor failed: {exit_status}");
            }
        }
        Err(err) => {
            println!("cargo:warning=failed to execute Asciidoctor: {err}");
        }
    }
}
