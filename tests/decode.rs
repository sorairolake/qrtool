// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod utils;

use predicates::prelude::predicate;

use crate::utils::command;

#[test]
fn basic_decode() {
    command::command()
        .arg("decode")
        .arg("data/basic/basic.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[test]
fn infer_subcommand_name_for_decode_command() {
    command::command()
        .arg("dec")
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("qrtool-decode"));
    command::command()
        .arg("d")
        .arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("qrtool-decode"));
}

#[test]
fn decode_from_non_existent_image_file() {
    let command = command::command()
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
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.bmp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.bmp"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("bmp")
        .arg("data/decode/decode.bmp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.dds")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.dds"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("dds")
        .arg("data/decode/decode.dds")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.ff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.ff"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("farbfeld")
        .arg("data/decode/decode.ff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.gif")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.gif"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("gif")
        .arg("data/decode/decode.gif")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.hdr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.hdr"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("hdr")
        .arg("data/decode/decode.hdr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/bmp.cur")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/bmp.cur"))
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/png.cur")
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/png.cur"))
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/bmp.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/bmp.ico"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/png.ico")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/png.ico"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.jpeg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.jpeg"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("jpeg")
        .arg("data/decode/decode.jpeg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.exr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.exr"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("openexr")
        .arg("data/decode/decode.exr")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.png"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("png")
        .arg("data/decode/decode.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/ascii.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/ascii.pbm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/ascii.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/ascii.pgm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/ascii.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/ascii.ppm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/binary.pbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/binary.pbm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/binary.pgm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/binary.pgm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/binary.ppm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/binary.ppm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.qoi")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.qoi"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("qoi")
        .arg("data/decode/decode.qoi")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.svg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.svg"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.svgz")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.svgz"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.tga")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.tga"))
        .assert()
        .failure()
        .code(69)
        .stderr(predicate::str::contains(
            "could not determine the image format",
        ));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("tga")
        .arg("data/decode/decode.tga")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/decode.tiff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.tiff"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("tiff")
        .arg("data/decode/decode.tiff")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/lossy.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/lossy.webp"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("data/decode/lossless.webp")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/lossless.webp"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
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
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("webp")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(65)
        .stderr(predicate::str::contains("could not read the image"));
}

#[cfg(feature = "decode-from-xbm")]
#[test]
fn decode_from_xbm() {
    command::command()
        .arg("decode")
        .arg("data/decode/decode.xbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .write_stdin(include_bytes!("data/decode/decode.xbm"))
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
    command::command()
        .arg("decode")
        .arg("-t")
        .arg("xbm")
        .arg("data/decode/decode.xbm")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));

    command::command()
        .arg("decode")
        .arg("-t")
        .arg("xbm")
        .arg("data/decode/decode.svg")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("could not create new XBM decoder"));
}

#[test]
fn decode_from_invalid_input_format() {
    command::command()
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
fn decode_with_invert() {
    command::command()
        .arg("decode")
        .arg("data/invert/basic.png")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[cfg(feature = "decode-from-svg")]
#[test]
fn decode_from_svg_with_invert() {
    command::command()
        .arg("decode")
        .arg("data/invert/basic.svg")
        .assert()
        .success()
        .stdout(predicate::eq("QR code"));
}

#[test]
fn decode_with_verbose() {
    command::command()
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
    command::command()
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
    command::command()
        .arg("decode")
        .arg("--verbose")
        .arg("--metadata")
        .assert()
        .failure()
        .code(2);
}
