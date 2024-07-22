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

use predicates::prelude::predicate;

#[test]
fn basic_decode() {
    utils::command::command()
        .arg("decode")
        .arg("data/basic/basic.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[test]
fn validate_aliases_for_decode_command() {
    utils::command::command()
        .arg("dec")
        .arg("-V")
        .assert()
        .success();
    utils::command::command()
        .arg("d")
        .arg("-V")
        .assert()
        .success();
}

#[test]
fn decode_from_non_existent_image_file() {
    let command = utils::command::command()
        .arg("decode")
        .arg("non_existent.png")
        .assert()
        .failure()
        .code(66)
        .stderr(predicate::str::contains(
            "could not read data from non_existent.png",
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
fn decode_from_non_image_file() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.txt")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
}

#[cfg(feature = "decode-from-bmp")]
#[test]
fn decode_from_bmp() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.bmp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.bmp"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("bmp")
        .arg("data/decode/decode.bmp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("bmp")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-dds")]
#[test]
fn decode_from_dds() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.dds")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.dds"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("dds")
        .arg("data/decode/decode.dds")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("dds")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-ff")]
#[test]
fn decode_from_farbfeld() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.ff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.ff"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("farbfeld")
        .arg("data/decode/decode.ff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("farbfeld")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-gif")]
#[test]
fn decode_from_gif() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.gif")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.gif"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("gif")
        .arg("data/decode/decode.gif")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("gif")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-hdr")]
#[test]
fn decode_from_hdr() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.hdr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.hdr"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("hdr")
        .arg("data/decode/decode.hdr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("hdr")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-ico")]
#[test]
fn decode_from_bmp_cur() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/bmp.cur")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/bmp.cur"))
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/bmp.cur")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-ico")]
#[test]
fn decode_from_png_cur() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/png.cur")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/png.cur"))
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/png.cur")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-ico")]
#[test]
fn decode_from_bmp_ico() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/bmp.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/bmp.ico"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/bmp.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-ico")]
#[test]
fn decode_from_png_ico() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/png.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/png.ico"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/png.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-ico")]
#[test]
fn decode_from_ico_with_wrong_format() {
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("ico")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-jpeg")]
#[test]
fn decode_from_jpeg() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.jpeg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.jpeg"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("jpeg")
        .arg("data/decode/decode.jpeg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("jpeg")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-exr")]
#[test]
fn decode_from_open_exr() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.exr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.exr"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("openexr")
        .arg("data/decode/decode.exr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("openexr")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_png() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.png"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("png")
        .arg("data/decode/decode.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("png")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-pnm")]
#[test]
fn decode_from_ascii_pbm() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/ascii.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/ascii.pbm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/ascii.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-pnm")]
#[test]
fn decode_from_ascii_pgm() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/ascii.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/ascii.pgm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/ascii.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-pnm")]
#[test]
fn decode_from_ascii_ppm() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/ascii.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/ascii.ppm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/ascii.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-pnm")]
#[test]
fn decode_from_binary_pbm() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/binary.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/binary.pbm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/binary.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-pnm")]
#[test]
fn decode_from_binary_pgm() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/binary.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/binary.pgm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/binary.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-pnm")]
#[test]
fn decode_from_binary_ppm() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/binary.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/binary.ppm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/binary.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-pnm")]
#[test]
fn decode_from_pnm_with_wrong_format() {
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("pnm")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-qoi")]
#[test]
fn decode_from_qoi() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.qoi")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.qoi"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("qoi")
        .arg("data/decode/decode.qoi")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("qoi")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-svg")]
#[test]
fn decode_from_svg() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.svg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.svg"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("svg")
        .arg("data/decode/decode.svg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-svg")]
#[test]
fn decode_from_svgz() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.svgz")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.svgz"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("svg")
        .arg("data/decode/decode.svgz")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-svg")]
#[test]
fn decode_from_svg_with_wrong_format() {
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("svg")
        .arg("data/decode/decode.png")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-tga")]
#[test]
fn decode_from_tga() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.tga")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.tga"))
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("tga")
        .arg("data/decode/decode.tga")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("tga")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-tiff")]
#[test]
fn decode_from_tiff() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/decode.tiff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.tiff"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("tiff")
        .arg("data/decode/decode.tiff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("tiff")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-webp")]
#[test]
fn decode_from_lossy_web_p() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/lossy.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/lossy.webp"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("webp")
        .arg("data/decode/lossy.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-webp")]
#[test]
fn decode_from_lossless_web_p() {
    utils::command::command()
        .arg("decode")
        .arg("data/decode/lossless.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/lossless.webp"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("webp")
        .arg("data/decode/lossless.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-webp")]
#[test]
fn decode_from_web_p_with_wrong_format() {
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("webp")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[test]
fn decode_from_invalid_input_format() {
    utils::command::command()
        .arg("decode")
        .arg("-t")
        .arg("a")
        .arg("data/decode/decode.png")
        .assert()
        .failure()
        .code(2)
        .stderr(predicate::str::contains(
            "invalid value 'a' for '--type <FORMAT>'",
        ));
}

#[test]
fn decode_with_verbose() {
    utils::command::command()
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
    utils::command::command()
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
    utils::command::command()
        .arg("decode")
        .arg("--verbose")
        .arg("--metadata")
        .assert()
        .failure()
        .code(2);
}

#[test]
fn long_version_for_decode_command() {
    utils::command::command()
        .arg("decode")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/long-version.md"
        )));
}

#[test]
fn after_long_help_for_decode_command() {
    utils::command::command()
        .arg("decode")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(include_str!(
            "assets/decode-after-long-help.md"
        )));
}
