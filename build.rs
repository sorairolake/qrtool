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
    path::Path,
    process::{Command, ExitStatus},
};

fn generate_man_page(out_dir: impl AsRef<Path>) -> io::Result<ExitStatus> {
    let man_dir = env::current_dir()?.join("doc/man/man1");
    let mut command = Command::new("asciidoctor");
    command
        .args(["-b", "manpage"])
        .args(["-a", concat!("revnumber=", env!("CARGO_PKG_VERSION"))]);
    #[cfg(feature = "decode-from-svg")]
    command.args(["-a", "decode-from-svg"]);
    command
        .args(["-D".as_ref(), out_dir.as_ref()])
        .args([
            man_dir.join("qrtool.1.adoc"),
            man_dir.join("qrtool-encode.1.adoc"),
            man_dir.join("qrtool-decode.1.adoc"),
            man_dir.join("qrtool-help.1.adoc"),
        ])
        .status()
}

fn main() {
    println!("cargo:rerun-if-changed=doc/man");

    let out_dir = env::var("OUT_DIR").expect("environment variable `OUT_DIR` not defined");
    match generate_man_page(out_dir) {
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
