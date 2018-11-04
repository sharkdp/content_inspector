# content_inspector

[![Crates.io](https://img.shields.io/crates/v/content_inspector.svg)](https://crates.io/crates/content_inspector)
[![Documentation](https://docs.rs/content_inspector/badge.svg)](https://docs.rs/content_inspector)

A simple library for *fast* inspection of binary buffers to guess the type of content.

This is mainly intended to quickly determine whether a given buffer contains "binary"
or "text" data. Programs like `grep` or `git diff` use similar mechanisms to decide whether
to treat some files as "binary data" or not.

The analysis is based on a very simple heuristic: Searching for NULL bytes
(indicating "binary" content) and the detection of special [byte order
marks](https://en.wikipedia.org/wiki/Byte_order_mark) (indicating a particular kind of textual
encoding). Note that **this analysis can fail**. For example, even if unlikely, UTF-8-encoded
text can legally contain NULL bytes. Conversely, some particular binary formats (like binary
[PGM](https://en.wikipedia.org/wiki/Netpbm_format)) may not contain NULL bytes. Also, for
performance reasons, only the first 1024 bytes are checked for the NULL-byte (if no BOM was
detected).

If this library reports a certain type of encoding (say `UTF_16LE`), there is **no guarantee** that
the binary buffer can actually be decoded as UTF-16LE.

## Usage

```rust
use content_inspector::{ContentType, inspect};

assert_eq!(ContentType::UTF_8, inspect(b"Hello"));
assert_eq!(ContentType::BINARY, inspect(b"\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00"));

assert!(inspect(b"Hello").is_text());
```

## CLI example

This crate also comes with a small example command-line program (see [`examples/inspect.rs`](examples/inspect.rs)) that demonstrates the usage:
```bash
> inspect
USAGE: inspect FILE [FILE...]

> inspect testdata/*
testdata/create_text_files.py: UTF-8
testdata/file_sources.md: UTF-8
testdata/test.jpg: binary
testdata/test.pdf: binary
testdata/test.png: binary
testdata/text_UTF-16BE-BOM.txt: UTF-16BE
testdata/text_UTF-16LE-BOM.txt: UTF-16LE
testdata/text_UTF-32BE-BOM.txt: UTF-32BE
testdata/text_UTF-32LE-BOM.txt: UTF-32LE
testdata/text_UTF-8-BOM.txt: UTF-8-BOM
testdata/text_UTF-8.txt: UTF-8
```

If you only want to detect whether something is a binary or text file, this is about a factor of 250 faster than `file --mime ...`.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
