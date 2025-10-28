// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use image::DynamicImage;
use predicates::prelude::predicate;

use crate::utils::command;

#[test]
fn basic_encode() {
    let output = command::command()
        .arg("encode")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/basic/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn infer_subcommand_name_for_encode_command() {
    command::command()
        .arg("enc")
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("qrtool-encode"));
    command::command()
        .arg("e")
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("qrtool-encode"));
}

#[test]
fn encode_if_output_is_directory() {
    let command = command::command()
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
    let command = command::command()
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
    let output = command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/encode/data.txt")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/basic/basic.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_non_existent_file() {
    let command = command::command()
        .arg("encode")
        .arg("-r")
        .arg("non_existent.txt")
        .assert()
        .failure()
        .code(66)
        .stderr(predicate::str::contains("could not open non_existent.txt"));
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
    let output = command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/module_size/3.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_svg_with_module_size() {
    command::command()
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
fn encode_to_pic_with_module_size() {
    command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("pic")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/module_size/3.pic")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_with_module_size() {
    command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("ansi")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/module_size/3_ansi.txt")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_256_with_module_size() {
    command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("ansi256")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/module_size/3_ansi_256.txt"
        )));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_true_color_with_module_size() {
    command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("ansi-true-color")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/module_size/3_ansi_true_color.txt"
        )));
}

#[test]
fn encode_to_ascii_with_module_size() {
    command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("ascii")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/module_size/3_ascii.txt")));
}

#[test]
fn encode_to_ascii_invert_with_module_size() {
    command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("ascii-invert")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/module_size/3_ascii_invert.txt"
        )));
}

#[test]
fn encode_to_unicode_with_module_size() {
    command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("unicode")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/module_size/3_unicode.txt"
        )));
}

#[test]
fn encode_to_unicode_invert_with_module_size() {
    command::command()
        .arg("encode")
        .arg("-s")
        .arg("3")
        .arg("-t")
        .arg("unicode-invert")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/module_size/3_unicode_invert.txt"
        )));
}

#[test]
fn encode_with_invalid_module_size() {
    command::command()
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
    command::command()
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
        let output = command::command()
            .arg("encode")
            .arg("-l")
            .arg("l")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_luma8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/level/low.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command::command()
            .arg("encode")
            .arg("-l")
            .arg("m")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_luma8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/level/medium.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command::command()
            .arg("encode")
            .arg("-l")
            .arg("q")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_luma8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/level/quartile.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command::command()
            .arg("encode")
            .arg("-l")
            .arg("h")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_luma8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/level/high.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn validate_alias_for_error_correction_level_option_of_encode_command() {
    let output = command::command()
        .arg("encode")
        .arg("--level")
        .arg("l")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/level/low.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_invalid_error_correction_level() {
    command::command()
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
    let output = command::command()
        .arg("encode")
        .arg("-v")
        .arg("40")
        .arg("--")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/symbol_version/40.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn validate_alias_for_symbol_version_option_of_encode_command() {
    let output = command::command()
        .arg("encode")
        .arg("--symversion")
        .arg("40")
        .arg("--")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/symbol_version/40.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_with_invalid_symbol_version() {
    command::command()
        .arg("encode")
        .arg("-v")
        .arg("0")
        .arg("--")
        .arg("QR code")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not set the version"))
        .stderr(predicate::str::contains("invalid version"));
    command::command()
        .arg("encode")
        .arg("-v")
        .arg("41")
        .arg("--")
        .arg("QR code")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not set the version"))
        .stderr(predicate::str::contains("invalid version"));
    command::command()
        .arg("encode")
        .arg("-v")
        .arg("17")
        .arg("139")
        .arg("7")
        .arg("--variant")
        .arg("rmqr")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "unexpected argument 'QR code' found",
        ));
}

#[test]
fn encode_with_margin() {
    let output = command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/margin/8.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_svg_with_margin() {
    command::command()
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
fn encode_to_pic_with_margin() {
    command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("pic")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/margin/8.pic")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_with_margin() {
    command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("ansi")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/margin/8_ansi.txt")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_256_with_margin() {
    command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("ansi256")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/margin/8_ansi_256.txt")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_true_color_with_margin() {
    command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("ansi-true-color")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/margin/8_ansi_true_color.txt"
        )));
}

#[test]
fn encode_to_ascii_with_margin() {
    command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("ascii")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/margin/8_ascii.txt")));
}

#[test]
fn encode_to_ascii_invert_with_margin() {
    command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("ascii-invert")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/margin/8_ascii_invert.txt"
        )));
}

#[test]
fn encode_to_unicode_with_margin() {
    command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("unicode")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/margin/8_unicode.txt")));
}

#[test]
fn encode_to_unicode_invert_with_margin() {
    command::command()
        .arg("encode")
        .arg("-m")
        .arg("8")
        .arg("-t")
        .arg("unicode-invert")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/margin/8_unicode_invert.txt"
        )));
}

#[test]
fn encode_with_invalid_margin() {
    command::command()
        .arg("encode")
        .arg("-m")
        .arg("-1")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains("unexpected argument '-1' found"));
    command::command()
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
    let output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_svg() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("svg")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/encode.svg")));
}

#[test]
fn encode_to_pic() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("pic")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/encode.pic")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/ansi.txt")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_256() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi256")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/ansi_256.txt")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_true_color() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi-true-color")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/encode/ansi_true_color.txt"
        )));
}

#[test]
fn encode_to_ascii() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ascii")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/ascii.txt")));
}

#[test]
fn encode_to_ascii_invert() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ascii-invert")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/ascii_invert.txt")));
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ASCIIi")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/ascii_invert.txt")));
}

#[test]
fn encode_to_unicode() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("unicode")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/unicode.txt")));
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("terminal")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/unicode.txt")));
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("UTF8")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/encode/unicode.txt")));
}

#[test]
fn encode_to_unicode_invert() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("unicode-invert")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/encode/unicode_invert.txt"
        )));
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("UTF8i")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/encode/unicode_invert.txt"
        )));
}

#[test]
fn encode_to_invalid_output_format() {
    command::command()
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

#[cfg(feature = "optimize-output-png")]
#[test]
fn encode_to_optimized_png() {
    let default_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("QR code")
        .output()
        .unwrap();

    let level0_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("0")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&level0_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(level0_output.stdout.len() <= default_output.stdout.len());
    assert!(level0_output.status.success());

    let level1_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("1")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&level1_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(level1_output.stdout.len() <= level0_output.stdout.len());
    assert!(level1_output.status.success());

    let level2_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("2")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&level2_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(level2_output.stdout.len() <= level1_output.stdout.len());
    assert!(level2_output.status.success());

    let level3_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("3")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&level3_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(level3_output.stdout.len() <= level2_output.stdout.len());
    assert!(level3_output.status.success());

    let level4_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("4")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&level4_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(level4_output.stdout.len() <= level3_output.stdout.len());
    assert!(level4_output.status.success());

    let level5_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("5")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&level5_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(level5_output.stdout.len() <= level4_output.stdout.len());
    assert!(level5_output.status.success());

    let level6_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("6")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&level6_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(level6_output.stdout.len() <= level5_output.stdout.len());
    assert!(level6_output.status.success());
}

#[cfg(feature = "optimize-output-png")]
#[test]
fn encode_to_maximum_optimized_png() {
    let level6_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("6")
        .arg("QR code")
        .output()
        .unwrap();

    let max_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("max")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&max_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert_eq!(max_output.stdout.len(), level6_output.stdout.len());
    assert!(max_output.status.success());
}

#[cfg(feature = "optimize-output-png")]
#[test]
fn encode_to_optimized_png_without_value() {
    let level2_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("2")
        .arg("QR code")
        .output()
        .unwrap();

    let without_value_output = command::command()
        .arg("encode")
        .arg("--optimize-png")
        .arg("-t")
        .arg("png")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&without_value_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert_eq!(
        without_value_output.stdout.len(),
        level2_output.stdout.len()
    );
    assert!(without_value_output.status.success());
}

#[cfg(feature = "optimize-output-png")]
#[test]
fn encode_to_optimized_png_with_invalid_level() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("7")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value '7' for '--optimize-png [<LEVEL>]'",
        ));
}

#[cfg(feature = "optimize-output-png")]
#[test]
fn encode_to_optimized_png_with_invalid_output_format() {
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("svg")
            .arg("--optimize-png")
            .arg("0")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains("output format is not PNG"));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("unicode")
            .arg("--optimize-png")
            .arg("0")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains("output format is not PNG"));
    }
}

#[cfg(feature = "optimize-output-png")]
#[test]
fn encode_to_optimized_png_using_zopfli() {
    let without_value_output = command::command()
        .arg("encode")
        .arg("--optimize-png")
        .arg("-t")
        .arg("png")
        .arg("QR code")
        .output()
        .unwrap();

    let zopfli_5_iterations_output = command::command()
        .arg("encode")
        .arg("-t")
        .arg("png")
        .arg("--optimize-png")
        .arg("--zopfli")
        .arg("5")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&zopfli_5_iterations_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(zopfli_5_iterations_output.stdout.len() < without_value_output.stdout.len());
    assert!(zopfli_5_iterations_output.status.success());

    let zopfli_default_iterations_output = command::command()
        .arg("encode")
        .arg("--optimize-png")
        .arg("--zopfli")
        .arg("-t")
        .arg("png")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&zopfli_default_iterations_output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/encode/encode.png").unwrap()
    );
    assert!(
        zopfli_default_iterations_output.stdout.len() < zopfli_5_iterations_output.stdout.len()
    );
    assert!(zopfli_default_iterations_output.status.success());
}

#[cfg(feature = "optimize-output-png")]
#[test]
fn encode_to_optimized_png_using_zopfli_without_level() {
    command::command()
        .arg("encode")
        .arg("--zopfli")
        .arg("-t")
        .arg("png")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "the following required arguments were not provided",
        ))
        .stderr(predicate::str::contains("--optimize-png [<LEVEL>]"));
}

#[cfg(feature = "optimize-output-png")]
#[test]
fn encode_to_optimized_png_using_zopfli_with_invalid_value() {
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("png")
            .arg("--optimize-png")
            .arg("--zopfli")
            .arg("0")
            .arg("QR code")
            .assert()
            .failure()
            .code(2)
            .stderr(predicate::str::contains(
                "invalid value '0' for '--zopfli [<ITERATION>]'",
            ))
            .stderr(predicate::str::contains(
                "number would be zero for non-zero type",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("png")
            .arg("--optimize-png")
            .arg("--zopfli")
            .arg("256")
            .arg("QR code")
            .assert()
            .failure()
            .code(2)
            .stderr(predicate::str::contains(
                "invalid value '256' for '--zopfli [<ITERATION>]'",
            ))
            .stderr(predicate::str::contains(
                "number too large to fit in target type",
            ));
    }
}

#[test]
fn encode_in_numeric_mode() {
    let output = command::command()
        .arg("encode")
        .arg("-v")
        .arg("1")
        .arg("--mode")
        .arg("numeric")
        .arg("42")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/mode/numeric.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_in_numeric_mode_max() {
    command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/numeric_max.txt")
        .arg("-l")
        .arg("l")
        .arg("-v")
        .arg("40")
        .arg("--mode")
        .arg("numeric")
        .assert()
        .success();
    command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/numeric_over_max.txt")
        .arg("-l")
        .arg("l")
        .arg("-v")
        .arg("40")
        .arg("--mode")
        .arg("numeric")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not construct a QR code"))
        .stderr(predicate::str::contains("data too long"));
}

#[test]
fn encode_in_alphanumeric_mode() {
    let output = command::command()
        .arg("encode")
        .arg("-v")
        .arg("1")
        .arg("--mode")
        .arg("alphanumeric")
        .arg("URL")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/mode/alphanumeric.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_in_alphanumeric_mode_max() {
    command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/alphanumeric_max.txt")
        .arg("-l")
        .arg("l")
        .arg("-v")
        .arg("40")
        .arg("--mode")
        .arg("alphanumeric")
        .assert()
        .success();
    command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/alphanumeric_over_max.txt")
        .arg("-l")
        .arg("l")
        .arg("-v")
        .arg("40")
        .arg("--mode")
        .arg("alphanumeric")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not construct a QR code"))
        .stderr(predicate::str::contains("data too long"));
}

#[test]
fn encode_in_byte_mode() {
    let output = command::command()
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
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/mode/byte.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_in_byte_mode_max() {
    command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/byte_max.txt")
        .arg("-l")
        .arg("l")
        .arg("-v")
        .arg("40")
        .arg("--mode")
        .arg("byte")
        .assert()
        .success();
    command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/byte_over_max.txt")
        .arg("-l")
        .arg("l")
        .arg("-v")
        .arg("40")
        .arg("--mode")
        .arg("byte")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not construct a QR code"))
        .stderr(predicate::str::contains("data too long"));
}

#[test]
fn encode_in_kanji_mode() {
    let output = command::command()
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
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/mode/kanji.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_in_kanji_mode_max() {
    command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/kanji_max.txt")
        .arg("-l")
        .arg("l")
        .arg("-v")
        .arg("40")
        .arg("--mode")
        .arg("kanji")
        .assert()
        .success();
    command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/mode/kanji_over_max.txt")
        .arg("-l")
        .arg("l")
        .arg("-v")
        .arg("40")
        .arg("--mode")
        .arg("kanji")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not construct a QR code"))
        .stderr(predicate::str::contains("data too long"));
}

#[test]
fn encode_with_invalid_mode() {
    command::command()
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
    command::command()
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
    let output = command::command()
        .arg("encode")
        .arg("--variant")
        .arg("normal")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/variant/normal.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_normal_qr_code_with_error_correction_level_and_symbol_version() {
    let output = command::command()
        .arg("encode")
        .arg("-l")
        .arg("h")
        .arg("-v")
        .arg("2")
        .arg("--variant")
        .arg("normal")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/variant/normal_2_h.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_normal_qr_code_with_extra_symbol_version() {
    let output = command::command()
        .arg("encode")
        .arg("-l")
        .arg("h")
        .arg("-v")
        .arg("2")
        .arg("1")
        .arg("--variant")
        .arg("normal")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/variant/normal_2_h.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_micro_qr_code() {
    let output = command::command()
        .arg("encode")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/variant/micro.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_micro_qr_code_with_error_correction_level_and_symbol_version() {
    let output = command::command()
        .arg("encode")
        .arg("-l")
        .arg("q")
        .arg("-v")
        .arg("4")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/variant/micro_4_q.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_micro_qr_code_with_extra_symbol_version() {
    let output = command::command()
        .arg("encode")
        .arg("-l")
        .arg("q")
        .arg("-v")
        .arg("4")
        .arg("3")
        .arg("--variant")
        .arg("micro")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/variant/micro_4_q.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_micro_qr_code_with_invalid_symbol_version() {
    command::command()
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
fn encode_as_rmqr_code() {
    let output = command::command()
        .arg("encode")
        .arg("--variant")
        .arg("rmqr")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/variant/rmqr.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_rmqr_code_with_error_correction_level_and_symbol_version() {
    let output = command::command()
        .arg("encode")
        .arg("-l")
        .arg("h")
        .arg("-v")
        .arg("15")
        .arg("43")
        .arg("--variant")
        .arg("rmqr")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_luma8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/variant/rmqr_r15x43_h.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_as_rmqr_code_with_invalid_symbol_version() {
    command::command()
        .arg("encode")
        .arg("-v")
        .arg("0")
        .arg("0")
        .arg("--variant")
        .arg("rmqr")
        .arg("QR code")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not set the version"))
        .stderr(predicate::str::contains("invalid version"));
    command::command()
        .arg("encode")
        .arg("-v")
        .arg("7")
        .arg("--variant")
        .arg("rmqr")
        .arg("QR code")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not set the version"))
        .stderr(predicate::str::contains("invalid version"));
}

#[test]
fn encode_with_invalid_variant() {
    command::command()
        .arg("encode")
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
fn encode_from_named_fg_color() {
    let output = command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("brown")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_rgb8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/colored/fg.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_named_bg_color() {
    let output = command::command()
        .arg("encode")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_rgb8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/colored/bg.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_named_color() {
    let output = command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("brown")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_rgb8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/colored/rgb.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_to_svg_from_named_color() {
    command::command()
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

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_from_named_color() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi")
        .arg("--foreground")
        .arg("brown")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(if cfg!(windows) {
            include_str!("data/colored/rgb_ansi_windows_10_console.txt")
        } else {
            include_str!("data/colored/rgb_ansi_vga.txt")
        }));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_256_from_named_color() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi256")
        .arg("--foreground")
        .arg("brown")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/colored/rgb_ansi_256.txt")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_true_color_from_named_color() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi-true-color")
        .arg("--foreground")
        .arg("brown")
        .arg("--background")
        .arg("lightslategray")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/colored/rgb_ansi_true_color.txt"
        )));
}

#[test]
fn encode_from_hex_fg_color() {
    {
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("#a52a2a")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/fg.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("a52a2a")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/fg.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hex_bg_color() {
    {
        let output = command::command()
            .arg("encode")
            .arg("--background")
            .arg("#778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/bg.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command::command()
            .arg("encode")
            .arg("--background")
            .arg("778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/bg.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hex_color() {
    {
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("#a52a2a")
            .arg("--background")
            .arg("#778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("a52a2a")
            .arg("--background")
            .arg("778899")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_to_svg_from_hex_color() {
    command::command()
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
    command::command()
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

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_from_hex_color() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi")
        .arg("--foreground")
        .arg("#a52a2a")
        .arg("--background")
        .arg("#778899")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(if cfg!(windows) {
            include_str!("data/colored/rgb_ansi_windows_10_console.txt")
        } else {
            include_str!("data/colored/rgb_ansi_vga.txt")
        }));
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi")
        .arg("--foreground")
        .arg("a52a2a")
        .arg("--background")
        .arg("778899")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(if cfg!(windows) {
            include_str!("data/colored/rgb_ansi_windows_10_console.txt")
        } else {
            include_str!("data/colored/rgb_ansi_vga.txt")
        }));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_256_from_hex_color() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi256")
        .arg("--foreground")
        .arg("#a52a2a")
        .arg("--background")
        .arg("#778899")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/colored/rgb_ansi_256.txt")));
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi256")
        .arg("--foreground")
        .arg("a52a2a")
        .arg("--background")
        .arg("778899")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!("data/colored/rgb_ansi_256.txt")));
}

#[cfg(feature = "output-as-ansi")]
#[test]
fn encode_to_ansi_true_color_from_hex_color() {
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi-true-color")
        .arg("--foreground")
        .arg("#a52a2a")
        .arg("--background")
        .arg("#778899")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/colored/rgb_ansi_true_color.txt"
        )));
    command::command()
        .arg("encode")
        .arg("-t")
        .arg("ansi-true-color")
        .arg("--foreground")
        .arg("a52a2a")
        .arg("--background")
        .arg("778899")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::eq(include_str!(
            "data/colored/rgb_ansi_true_color.txt"
        )));
}

#[test]
fn encode_from_hex_color_with_alpha() {
    {
        let output = command::command()
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
        let output = command::command()
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
fn encode_from_short_hex_color() {
    {
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("#111")
            .arg("--background")
            .arg("#eee")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/rgb_short.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("111")
            .arg("--background")
            .arg("eee")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/rgb_short.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_short_hex_color_with_alpha() {
    {
        let output = command::command()
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
        let output = command::command()
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
    command::command()
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
    command::command()
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
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165 42 42)")
            .arg("--background")
            .arg("rgb(119 136 153)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("rgb(165, 42, 42)")
            .arg("--background")
            .arg("rgb(119, 136, 153)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/rgb.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_rgb_color_with_alpha() {
    {
        let output = command::command()
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
        let output = command::command()
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
        let output = command::command()
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
        let output = command::command()
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
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_invalid_rgb_bg_color() {
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_hsl_color() {
    {
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248 39% 39.2%)")
            .arg("--background")
            .arg("hsl(0 0% 66.3%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/hsl.png").unwrap()
        );
        assert!(output.status.success());
    }
    {
        let output = command::command()
            .arg("encode")
            .arg("--foreground")
            .arg("hsl(248, 39%, 39.2%)")
            .arg("--background")
            .arg("hsl(0, 0%, 66.3%)")
            .arg("QR code")
            .output()
            .unwrap();
        assert_eq!(
            image::load_from_memory(&output.stdout)
                .map(DynamicImage::into_rgb8)
                .map(DynamicImage::from)
                .unwrap(),
            image::open("tests/data/colored/hsl.png").unwrap()
        );
        assert!(output.status.success());
    }
}

#[test]
fn encode_from_hsl_color_with_alpha() {
    {
        let output = command::command()
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
        let output = command::command()
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
        let output = command::command()
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
        let output = command::command()
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
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_invalid_hsl_bg_color() {
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_hwb_color() {
    let output = command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("hwb(50.6 0% 0%)")
        .arg("--background")
        .arg("hwb(0 66.3% 33.7%)")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_rgb8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/colored/hwb.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_hwb_color_with_alpha() {
    let output = command::command()
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
        image::open("tests/data/colored/hwb_with_alpha.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_invalid_hwb_fg_color() {
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_invalid_hwb_bg_color() {
    command::command()
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
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_oklab_color() {
    let output = command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("oklab(50.4% -0.0906 0.0069)")
        .arg("--background")
        .arg("oklab(61.9% -0.0120 -0.0302)")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_rgb8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/colored/oklab.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_oklab_color_with_alpha() {
    let output = command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("oklab(50.4% -0.0906 0.0069 / 0.5)")
        .arg("--background")
        .arg("oklab(61.9% -0.0120 -0.0302 / 0.5)")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout).unwrap(),
        image::open("tests/data/colored/oklab_with_alpha.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_invalid_oklab_fg_color() {
    command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("oklab(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'oklab(0)' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_invalid_oklab_bg_color() {
    command::command()
        .arg("encode")
        .arg("--background")
        .arg("oklab(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'oklab(0)' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_oklch_color() {
    let output = command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("oklch(59.41% 0.16 301.29)")
        .arg("--background")
        .arg("oklch(61.9% 0.032 248.35)")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout)
            .map(DynamicImage::into_rgb8)
            .map(DynamicImage::from)
            .unwrap(),
        image::open("tests/data/colored/oklch.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_oklch_color_with_alpha() {
    let output = command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("oklch(59.41% 0.16 301.29 / 49.8%)")
        .arg("--background")
        .arg("oklch(61.9% 0.032 248.35 / 49.8%)")
        .arg("QR code")
        .output()
        .unwrap();
    assert_eq!(
        image::load_from_memory(&output.stdout).unwrap(),
        image::open("tests/data/colored/oklch_with_alpha.png").unwrap()
    );
    assert!(output.status.success());
}

#[test]
fn encode_from_invalid_oklch_fg_color() {
    command::command()
        .arg("encode")
        .arg("--foreground")
        .arg("oklch(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'oklch(0)' for '--foreground <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_invalid_oklch_bg_color() {
    command::command()
        .arg("encode")
        .arg("--background")
        .arg("oklch(0)")
        .arg("QR code")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'oklch(0)' for '--background <COLOR>'",
        ))
        .stderr(predicate::str::contains("invalid color function"));
}

#[test]
fn encode_from_invalid_fg_color_function() {
    command::command()
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
    command::command()
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
    command::command()
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
    command::command()
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
fn encode_with_colors_to_pic() {
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("pic")
            .arg("--foreground")
            .arg("brown")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("pic")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("pic")
            .arg("--foreground")
            .arg("brown")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
}

#[test]
fn encode_with_colors_to_ascii() {
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("ascii")
            .arg("--foreground")
            .arg("brown")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("ascii")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("ascii")
            .arg("--foreground")
            .arg("brown")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
}

#[test]
fn encode_with_colors_to_ascii_invert() {
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("ascii-invert")
            .arg("--foreground")
            .arg("brown")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("ascii-invert")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("ascii-invert")
            .arg("--foreground")
            .arg("brown")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
}

#[test]
fn encode_with_colors_to_unicode() {
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("unicode")
            .arg("--foreground")
            .arg("brown")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("unicode")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("unicode")
            .arg("--foreground")
            .arg("brown")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
}

#[test]
fn encode_with_colors_to_unicode_invert() {
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("unicode-invert")
            .arg("--foreground")
            .arg("brown")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("unicode-invert")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
    {
        command::command()
            .arg("encode")
            .arg("-t")
            .arg("unicode-invert")
            .arg("--foreground")
            .arg("brown")
            .arg("--background")
            .arg("lightslategray")
            .arg("QR code")
            .assert()
            .failure()
            .code(1)
            .stderr(predicate::str::contains(
                "foreground and/or background colors cannot be changed",
            ));
    }
}

#[test]
fn encode_with_verbose() {
    command::command()
        .arg("encode")
        .arg("--verbose")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: 1\nLevel: M\n"));
    command::command()
        .arg("encode")
        .arg("-l")
        .arg("h")
        .arg("--verbose")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: 1\nLevel: H\n"));

    command::command()
        .arg("encode")
        .arg("--variant")
        .arg("micro")
        .arg("--verbose")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: 3\nLevel: M\n"));
    command::command()
        .arg("encode")
        .arg("-l")
        .arg("q")
        .arg("--variant")
        .arg("micro")
        .arg("--verbose")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: 4\nLevel: Q\n"));

    command::command()
        .arg("encode")
        .arg("--variant")
        .arg("rmqr")
        .arg("--verbose")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: R13x27\nLevel: M\n"));
    command::command()
        .arg("encode")
        .arg("-l")
        .arg("h")
        .arg("--variant")
        .arg("rmqr")
        .arg("--verbose")
        .arg("QR code")
        .assert()
        .success()
        .stdout(predicate::ne(&[] as &[u8]))
        .stderr(predicate::eq("Version: R11x43\nLevel: H\n"));
}

#[test]
fn validate_the_options_dependencies_for_encode_command() {
    command::command()
        .arg("encode")
        .arg("-r")
        .arg("data/encode/data.txt")
        .arg("QR code")
        .assert()
        .failure()
        .code(2);
}
