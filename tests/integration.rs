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
        image_for_encoding::DynamicImage::ImageLuma8(
            image_for_encoding::load_from_memory(&output.stdout)
                .unwrap()
                .to_luma8()
        ),
        image_for_encoding::open("tests/data/basic/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_data_from_file() {
    let output = command()
        .arg("encode")
        .arg("-r")
        .arg("data/encode/data.txt")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::DynamicImage::ImageLuma8(
            image_for_encoding::load_from_memory(&output.stdout)
                .unwrap()
                .to_luma8()
        ),
        image_for_encoding::open("tests/data/basic/basic.png").unwrap()
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
        image_for_encoding::DynamicImage::ImageLuma8(
            image_for_encoding::load_from_memory(&output.stdout)
                .unwrap()
                .to_luma8()
        ),
        image_for_encoding::open("tests/data/level/low.png").unwrap()
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
        image_for_encoding::DynamicImage::ImageLuma8(
            image_for_encoding::load_from_memory(&output.stdout)
                .unwrap()
                .to_luma8()
        ),
        image_for_encoding::open("tests/data/level/quartile.png").unwrap()
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
        image_for_encoding::DynamicImage::ImageLuma8(
            image_for_encoding::load_from_memory(&output.stdout)
                .unwrap()
                .to_luma8()
        ),
        image_for_encoding::open("tests/data/level/high.png").unwrap()
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
        image_for_encoding::DynamicImage::ImageLuma8(
            image_for_encoding::load_from_memory(&output.stdout)
                .unwrap()
                .to_luma8()
        ),
        image_for_encoding::open("tests/data/margin/8.png").unwrap()
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
        image_for_encoding::DynamicImage::ImageLuma8(
            image_for_encoding::load_from_memory(&output.stdout)
                .unwrap()
                .to_luma8()
        ),
        image_for_encoding::open("tests/data/variant/micro.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_colored() {
    let output = command()
        .arg("encode")
        .arg("--foreground")
        .arg("#bc002d")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/colored/fg.png").unwrap()
    );
    assert!(output.status.success());

    let output = command()
        .arg("encode")
        .arg("--background")
        .arg("#7d8694")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/colored/bg.png").unwrap()
    );
    assert!(output.status.success());

    let output = command()
        .arg("encode")
        .arg("--foreground")
        .arg("#bc002d")
        .arg("--background")
        .arg("#7d8694")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/colored/rgb.png").unwrap()
    );
    assert!(output.status.success());

    let output = command()
        .arg("encode")
        .arg("--foreground")
        .arg("bc002d7f")
        .arg("--background")
        .arg("7d86947f")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/colored/rgba.png").unwrap()
    );
    assert!(output.status.success());

    let output = command()
        .arg("encode")
        .arg("--foreground")
        .arg("#111")
        .arg("--background")
        .arg("#eee")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/colored/rgb_short.png").unwrap()
    );
    assert!(output.status.success());

    let output = command()
        .arg("encode")
        .arg("--foreground")
        .arg("1118")
        .arg("--background")
        .arg("eee8")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image_for_encoding::load_from_memory(&output.stdout).unwrap(),
        image_for_encoding::open("tests/data/colored/rgba_short.png").unwrap()
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
        .arg("data/encode/data.txt")
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
        .arg("data/basic/basic.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code\n"));
}

#[test]
#[cfg(feature = "decode-from-svg")]
fn decode_from_svg() {
    command()
        .arg("decode")
        .arg("data/decode/decode.svg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code\n"));
}

#[test]
#[cfg(feature = "decode-from-svg")]
fn decode_from_svgz() {
    command()
        .arg("decode")
        .arg("data/decode/decode.svgz")
        .assert()
        .success()
        .stdout(predicate::eq("QR code\n"));
}

#[test]
fn decode_with_verbose() {
    command()
        .arg("decode")
        .arg("--verbose")
        .arg("data/basic/basic.png")
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
        .arg("data/basic/basic.png")
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
