# qrtool

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
![License][license-badge]

**qrtool** is a command-line utility for encoding or decoding QR code.

## Installation

### From source

```sh
cargo install qrtool
```

### From binaries

The [release page][release-page-url] contains pre-built binaries for Linux,
macOS and Windows.

### How to build

Please see [BUILD.adoc](BUILD.adoc).

## Usage

### Basic usage

Encode a string in a QR code:

```sh
qrtool encode "QR code" > output.png
```

Generate this image:

![Output](tests/data/basic/basic.png)

Decode a QR code from this image:

```sh
qrtool decode output.png
```

Output:

```text
QR code
```

### SVG generation

```sh
qrtool encode -o output.svg -t svg "QR code"
```

Generate this image:

![Output](tests/data/decode/decode.svg)

### Micro QR code generation

```sh
qrtool encode -v 3 --variant micro "QR code" > output.png
```

Generate this image:

![Output](tests/data/variant/micro.png)

### Colored output

```sh
qrtool encode --foreground "#bc002d" "QR code" > output.png
```

Generate this image:

![Output](tests/data/colored/fg.png)

### Convert to and from unsupported image formats

Both `encode` and `decode` can read from stdin and output to stdout.

Example (convert to AVIF):

```sh
cat Cargo.toml | qrtool encode | magick png:- output.avif
```

Example (convert from AVIF):

```sh
magick output.avif png:- | qrtool decode | bat -l toml
```

### Generate shell completion

`--generate-completion` option generates shell completions to stdout.

The following shells are supported:

- `bash`
- `elvish`
- `fish`
- `powershell`
- `zsh`

Example:

```sh
qrtool --generate-completion bash > qrtool.bash
```

## Command-line options

Please see the following:

- [`qrtool(1)`](doc/man/man1/qrtool.1.adoc)
- [`qrtool-encode(1)`](doc/man/man1/qrtool-encode.1.adoc)
- [`qrtool-decode(1)`](doc/man/man1/qrtool-decode.1.adoc)

## Changelog

Please see [CHANGELOG.adoc](CHANGELOG.adoc).

## Contributing

Please see [CONTRIBUTING.adoc](CONTRIBUTING.adoc).

## License

Copyright (C) 2022 Shun Sakai (see [AUTHORS.adoc](AUTHORS.adoc))

This program is distributed under the terms of either the _Apache License 2.0_
or the _MIT License_.

See [COPYRIGHT](COPYRIGHT), [LICENSE-APACHE](LICENSE-APACHE) and
[LICENSE-MIT](LICENSE-MIT) for more details.

[ci-badge]: https://github.com/sorairolake/qrtool/workflows/CI/badge.svg
[ci-url]: https://github.com/sorairolake/qrtool/actions?query=workflow%3ACI
[version-badge]: https://img.shields.io/crates/v/qrtool
[version-url]: https://crates.io/crates/qrtool
[license-badge]: https://img.shields.io/crates/l/qrtool
[release-page-url]: https://github.com/sorairolake/qrtool/releases
