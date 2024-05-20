// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]
#![allow(clippy::multiple_crate_versions)]

mod utils;

use image::DynamicImage;
use predicates::prelude::predicate;

#[test]
fn basic_encode() {
    let output = utils::command::command()
        .arg("encode")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/basic/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn validate_aliases_for_encode_command() {
    utils::command::command()
        .arg("enc")
        .arg("-V")
        .assert()
        .success();
    utils::command::command()
        .arg("e")
        .arg("-V")
        .assert()
        .success();
}

#[test]
fn encode_if_output_is_directory() {
    let command = utils::command::command()
        .arg("encode")
        .arg("-o")
        .arg("data/dummy")
        .arg("QR code")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "could not write the image to data/dummy",
        ));
    if cfg!(windows) {
        command.stderr(predicate::str::contains("Access is denied. (os error 5)"));
    } else {
        command.stderr(predicate::str::contains("Is a directory (os error 21)"));
    }
}

#[test]
fn encode_to_svg_if_output_is_directory() {
    let command = utils::command::command()
        .arg("encode")
        .arg("-o")
        .arg("data/dummy")
        .arg("-t")
        .arg("svg")
        .arg("QR code")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "could not write the image to data/dummy",
        ));
    if cfg!(windows) {
        command.stderr(predicate::str::contains("Access is denied. (os error 5)"));
    } else {
        command.stderr(predicate::str::contains("Is a directory (os error 21)"));
    }
}

#[test]
fn encode_from_file() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/encode/data.txt")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/basic/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_non_existent_file() {
    let command = utils::command::command()
        .arg("encode")
        .arg("-r")
        .arg("non_existent.txt")
        .assert()
        .failure()
        .code(66)
        .stderr(predicate::str::contains(
            "could not read data from non_existent.txt",
        ));
    if cfg!(windows) {
        command.stderr(predicate::str::contains(
            "The system cannot find the file specified. (os error 2)",
        ));
    } else {
        command.stderr(predicate::str::contains(
            "No such file or directory (os error 2)",
        ));
    }
}

#[test]
fn encode_with_module_size() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/module_size/3.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_svg_with_module_size() {
    utils::command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("svg")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/module_size/3.svg")));
}

#[test]
fn encode_to_terminal_with_module_size() {
    utils::command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("terminal")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/module_size/3.txt")));
}

#[test]
fn encode_with_invalid_module_size() {
    utils::command::command()
        .arg("encode")
        .arg("-s")
        .arg("0")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '0' for '--size <NUMBER>'",
        ))
        .stderr(predicate::str::contains(
            "number would be zero for non-zero type",
        ));
    utils::command::command()
        .arg("encode")
        .arg("-s")
        .arg("4294967296")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '4294967296' for '--size <NUMBER>'",
        ))
        .stderr(predicate::str::contains(
            "number too large to fit in target type",
        ));
}

#[test]
fn encode_with_error_correction_level() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("-l")
            .arg("l")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
            image::open("tests/data/level/low.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("-l")
            .arg("m")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
            image::open("tests/data/level/medium.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("-l")
            .arg("q")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
            image::open("tests/data/level/quartile.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("-l")
            .arg("h")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
            image::open("tests/data/level/high.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn validate_alias_for_error_correction_level_option_of_encode_command() {
    let output = utils::command::command()
        .arg("encode")
        .arg("--level")
        .arg("l")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/level/low.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_invalid_error_correction_level() {
    utils::command::command()
        .arg("encode")
        .arg("-l")
        .arg("a")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--error-correction-level <LEVEL>'",
        ));
}

#[test]
fn encode_with_symbol_version() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("40")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/symbol_version/40.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn validate_alias_for_symbol_version_option_of_encode_command() {
    let output = utils::command::command()
        .arg("encode")
        .arg("--symversion")
        .arg("40")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/symbol_version/40.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_invalid_symbol_version() {
    utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("0")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '0' for '--symbol-version <NUMBER>'",
        ))
        .stderr(predicate::str::contains("0 is not in 1..=40"));
    utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("41")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '41' for '--symbol-version <NUMBER>'",
        ))
        .stderr(predicate::str::contains("41 is not in 1..=40"));
}

#[test]
fn encode_with_margin() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/margin/8.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_svg_with_margin() {
    utils::command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("svg")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/margin/8.svg")));
}

#[test]
fn encode_to_terminal_with_margin() {
    utils::command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("terminal")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/margin/8.txt")));
}

#[test]
fn encode_with_invalid_margin() {
    utils::command::command()
        .arg("encode")
        .arg("-m")
        .arg("-1")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("unexpected argument '-1' found"));
    utils::command::command()
        .arg("encode")
        .arg("-m")
        .arg("4294967296")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '4294967296' for '--margin <NUMBER>'",
        ))
        .stderr(predicate::str::contains(
            "4294967296 is not in 0..=4294967295",
        ));
}

#[test]
fn encode_to_png() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_svg() {
    utils::command::command()
        .arg("encode")
        .arg("-t")
        .arg("svg")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/encode.svg")));
}

#[test]
fn encode_to_terminal() {
    utils::command::command()
        .arg("encode")
        .arg("-t")
        .arg("terminal")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/encode.txt")));
}

#[test]
fn encode_to_invalid_output_format() {
    utils::command::command()
        .arg("encode")
        .arg("-t")
        .arg("a")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--type <FORMAT>'",
        ));
}

#[test]
fn encode_in_numeric_mode() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("1")
        .arg("--mode")
        .arg("numeric")
        .arg("42")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/mode/numeric.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_in_alphanumeric_mode() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("1")
        .arg("--mode")
        .arg("alphanumeric")
        .arg("URL")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/mode/alphanumeric.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_in_byte_mode() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/byte.txt")
        .arg("-v")
        .arg("1")
        .arg("--mode")
        .arg("byte")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/mode/byte.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_in_kanji_mode() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/kanji.txt")
        .arg("-v")
        .arg("1")
        .arg("--mode")
        .arg("kanji")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/mode/kanji.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_invalid_mode() {
    utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("1")
        .arg("--mode")
        .arg("a")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--mode <MODE>'",
        ));
}

#[test]
fn encode_with_mode_without_symbol_version() {
    utils::command::command()
        .arg("encode")
        .arg("--mode")
        .arg("numeric")
        .arg("42")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the following required arguments were not provided",
        ))
        .stderr(predicate::str::contains("--symbol-version <NUMBER>"));
}

#[test]
fn encode_as_normal_qr_code() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("1")
        .arg("--variant")
        .arg("normal")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/variant/normal.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_micro_qr_code() {
    let output = utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("3")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageLuma8(image::load_from_memory(&output.stdout).unwrap().to_luma8()),
        image::open("tests/data/variant/micro.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_micro_qr_code_with_invalid_symbol_version() {
    utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("5")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not set the version"))
        .stderr(predicate::str::contains("invalid version"));
}

#[test]
fn encode_with_invalid_variant() {
    utils::command::command()
        .arg("encode")
        .arg("-v")
        .arg("3")
        .arg("--variant")
        .arg("a")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--variant <TYPE>'",
        ));
}

#[test]
fn encode_with_variant_without_symbol_version() {
    utils::command::command()
        .arg("encode")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the following required arguments were not provided",
        ))
        .stderr(predicate::str::contains("--symbol-version <NUMBER>"));
}

#[test]
fn encode_from_named_fg_color() {
    let output = utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("brown")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
        image::open("tests/data/colored/fg.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_named_bg_color() {
    let output = utils::command::command()
        .arg("encode")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
        image::open("tests/data/colored/bg.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_named_color() {
    let output = utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("brown")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
        image::open("tests/data/colored/rgb.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_svg_from_named_color() {
    utils::command::command()
        .arg("encode")
        .arg("-t")
        .arg("svg")
        .arg("--foreground")
        .arg("brown")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/colored/rgb.svg")));
}

#[test]
fn encode_from_hex_fg_color() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("#a52a2a")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/fg.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("a52a2a")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/fg.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hex_bg_color() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--background")
            .arg("#778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/bg.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--background")
            .arg("778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/bg.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hex_color() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("#a52a2a")
            .arg("--background")
            .arg("#778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("a52a2a")
            .arg("--background")
            .arg("778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_to_svg_from_hex_color() {
    utils::command::command()
        .arg("encode")
        .arg("-t")
        .arg("svg")
        .arg("--foreground")
        .arg("#a52a2a")
        .arg("--background")
        .arg("#778899")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/colored/rgb.svg")));
    utils::command::command()
        .arg("encode")
        .arg("-t")
        .arg("svg")
        .arg("--foreground")
        .arg("a52a2a")
        .arg("--background")
        .arg("778899")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/colored/rgb.svg")));
}

#[test]
fn encode_from_hex_color_with_alpha() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("#a52a2a7f")
            .arg("--background")
            .arg("#7788997f")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("a52a2a7f")
            .arg("--background")
            .arg("7788997f")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_to_svg_from_hex_color_with_alpha() {
    utils::command::command()
        .arg("encode")
        .arg("-t")
        .arg("svg")
        .arg("--foreground")
        .arg("#a52a2a7f")
        .arg("--background")
        .arg("#7788997f")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/colored/rgba.svg")));
    utils::command::command()
        .arg("encode")
        .arg("-t")
        .arg("svg")
        .arg("--foreground")
        .arg("a52a2a7f")
        .arg("--background")
        .arg("7788997f")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/colored/rgba.svg")));
}

#[test]
fn encode_from_short_hex_color() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("#111")
            .arg("--background")
            .arg("#eee")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb_short.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("111")
            .arg("--background")
            .arg("eee")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb_short.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_short_hex_color_with_alpha() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("#1118")
            .arg("--background")
            .arg("#eee8")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba_short.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("1118")
            .arg("--background")
            .arg("eee8")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba_short.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_invalid_hex_fg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("#g")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '#g' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hex format"));
}

#[test]
fn encode_from_invalid_hex_bg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--background")
        .arg("#g")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '#g' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hex format"));
}

#[test]
fn encode_from_rgb_color() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165 42 42)")
            .arg("--background")
            .arg("rgb(119 136 153)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165, 42, 42)")
            .arg("--background")
            .arg("rgb(119, 136, 153)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_rgb_color_with_alpha() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165 42 42 / 49.8%)")
            .arg("--background")
            .arg("rgb(119 136 153 / 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165, 42, 42, 49.8%)")
            .arg("--background")
            .arg("rgb(119, 136, 153, 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_rgba_color() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgba(165 42 42 / 49.8%)")
            .arg("--background")
            .arg("rgba(119 136 153 / 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgba(165, 42, 42, 49.8%)")
            .arg("--background")
            .arg("rgba(119, 136, 153, 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/rgba.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_invalid_rgb_fg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("rgb(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'rgb(0)' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid rgb format"));
    utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("rgba(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'rgba(0)' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid rgb format"));
}

#[test]
fn encode_from_invalid_rgb_bg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--background")
        .arg("rgb(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'rgb(0)' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid rgb format"));
    utils::command::command()
        .arg("encode")
        .arg("--background")
        .arg("rgba(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'rgba(0)' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid rgb format"));
}

#[test]
fn encode_from_hsl_color() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248 39% 39.2%)")
            .arg("--background")
            .arg("hsl(0 0% 66.3%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/hsl.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248, 39%, 39.2%)")
            .arg("--background")
            .arg("hsl(0, 0%, 66.3%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
            image::open("tests/data/colored/hsl.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hsl_color_with_alpha() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248 39% 39.2% / 49.8%)")
            .arg("--background")
            .arg("hsl(0 0% 66.3% / 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/hsla.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248, 39%, 39.2%, 49.8%)")
            .arg("--background")
            .arg("hsl(0, 0%, 66.3%, 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/hsla.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hsla_color() {
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsla(248 39% 39.2% / 49.8%)")
            .arg("--background")
            .arg("hsla(0 0% 66.3% / 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/hsla.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = utils::command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsla(248, 39%, 39.2%, 49.8%)")
            .arg("--background")
            .arg("hsla(0, 0%, 66.3%, 49.8%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout).unwrap(),
            image::open("tests/data/colored/hsla.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_invalid_hsl_fg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("hsl(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hsl(0)' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hsl format"));
    utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("hsla(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hsla(0)' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hsl format"));
}

#[test]
fn encode_from_invalid_hsl_bg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--background")
        .arg("hsl(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hsl(0)' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hsl format"));
    utils::command::command()
        .arg("encode")
        .arg("--background")
        .arg("hsla(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hsla(0)' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hsl format"));
}

#[test]
fn encode_from_hwb_color() {
    let output = utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("hwb(50.6 0% 0%)")
        .arg("--background")
        .arg("hwb(0 66.3% 33.7%)")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        DynamicImage::ImageRgb8(image::load_from_memory(&output.stdout).unwrap().to_rgb8()),
        image::open("tests/data/colored/hwb.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_hwb_color_with_alpha() {
    let output = utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("hwb(50.6 0% 0% / 49.8%)")
        .arg("--background")
        .arg("hwb(0 66.3% 33.7% / 49.8%)")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout).unwrap(),
        image::open("tests/data/colored/hwba.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_invalid_hwb_fg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("hwb(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hwb(0)' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hwb format"));
}

#[test]
fn encode_from_invalid_hwb_bg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--background")
        .arg("hwb(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'hwb(0)' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid hwb format"));
}

#[test]
fn encode_from_invalid_fg_color_function() {
    utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("fn(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'fn(0)' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_invalid_bg_color_function() {
    utils::command::command()
        .arg("encode")
        .arg("--background")
        .arg("fn(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'fn(0)' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_unknown_fg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("a")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid unknown format"));
}

#[test]
fn encode_from_unknown_bg_color() {
    utils::command::command()
        .arg("encode")
        .arg("--background")
        .arg("a")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid unknown format"));
}

#[test]
fn encode_with_verbose() {
    utils::command::command()
        .arg("encode")
        .arg("--verbose")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: 1\nLevel: M\n"));
}

#[test]
fn long_version_for_encode_command() {
    utils::command::command()
        .arg("encode")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/long-version.md"
        )));
}

#[test]
fn after_long_help_for_encode_command() {
    utils::command::command()
        .arg("encode")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/encode-after-long-help.md"
        )));
}

#[test]
fn validate_the_options_dependencies_for_encode_command() {
    utils::command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/encode/data.txt")
        .arg("QR code")
        .assert()
        .failure()
        .code(2);
    utils::command::command()
        .arg("encode")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .assert()
        .failure()
        .code(2);
}
