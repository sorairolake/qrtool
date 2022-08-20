//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

// Lint levels of rustc.
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]

use assert_cmd::Command;
use predicates::prelude::predicate;

fn command() -> Command {
    let mut command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    command.current_dir("tests");
    command
}

#[test]
fn generate_completion_conflicts_with_subcommands() {
    command()
        .arg("--generate-completion")
        .arg("bash")
        .arg("encode")
        .assert()
        .failure()
        .code(2);
    command()
        .arg("--generate-completion")
        .arg("bash")
        .arg("decode")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn basic_encode() {
    let output = command().arg("encode").arg("QR code").output().unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_data_from_file() {
    let output = command()
        .arg("encode")
        .arg("-r")
        .arg("data/data.txt")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_error_correction_level() {
    let output = command()
        .arg("encode")
        .arg("-l")
        .arg("l")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/low.png").unwrap()
    );
    assert!(output.status.success());

    let output = command()
        .arg("encode")
        .arg("-l")
        .arg("q")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/quartile.png").unwrap()
    );
    assert!(output.status.success());

    let output = command()
        .arg("encode")
        .arg("-l")
        .arg("h")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/high.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_margin() {
    let output = command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/8.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_micro_qr_code() {
    let output = command()
        .arg("encode")
        .arg("-v")
        .arg("3")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/micro.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_verbose() {
    command()
        .arg("encode")
        .arg("--verbose")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: 1\nLevel: M\n"));
}

#[test]
fn validate_the_options_dependencies_for_encode_command() {
    command()
        .arg("encode")
        .arg("-r")
        .arg("data/data.txt")
        .arg("QR code")
        .assert()
        .failure()
        .code(2);

    command()
        .arg("encode")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn basic_decode() {
    command()
        .arg("decode")
        .arg("data/basic.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code\n"));
}

#[test]
#[cfg(feature = "decode-from-svg")]
fn decode_from_svg() {
    command()
        .arg("decode")
        .arg("data/basic.svg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code\n"));
}

#[test]
#[cfg(feature = "decode-from-svg")]
fn decode_from_svgz() {
    command()
        .arg("decode")
        .arg("data/basic.svgz")
        .assert()
        .success()
        .stdout(predicate::eq("QR code\n"));
}

#[test]
fn decode_with_verbose() {
    command()
        .arg("decode")
        .arg("--verbose")
        .arg("data/basic.png")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: 1\nLevel: M\n"));
}

#[test]
fn decode_with_metadata() {
    command()
        .arg("decode")
        .arg("--metadata")
        .arg("data/basic.png")
        .assert()
        .success()
        .stdout(predicate::eq(&[] as &[u8]))
        .stderr(predicate::eq("Version: 1\nLevel: M\n"));
}

#[test]
fn validate_the_options_dependencies_for_decode_command() {
    command()
        .arg("decode")
        .arg("--verbose")
        .arg("--metadata")
        .assert()
        .failure()
        .code(2);
}
