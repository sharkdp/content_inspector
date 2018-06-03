# content_inspector

[![Crates.io](https://img.shields.io/crates/v/content_inspector.svg)](https://crates.io/crates/content_inspector)
[![Documentation](https://docs.rs/content_inspector/badge.svg)](https://docs.rs/content_inspector)

A simple library for fast inspection of binary buffers to guess/determine the type of content.

This is mainly intended to quickly determine whether a given buffer contains "binary" or "text"
data. The analysis is based on a very simple heuristic: Detection of special [byte order
marks](https://en.wikipedia.org/wiki/Byte_order_mark) and searching for NULL bytes. Note that
this analysis can fail. For example, even if unlikely, UTF-8-encoded text can legally contain
NULL bytes. Also, for performance reasons, only the first *1024* bytes are checked for the
NULL-byte (if no BOM) is detected.

## Usage

``` rust
use content_inspector::{ContentType, inspect};

assert_eq!(ContentType::UTF_8, inspect(b"Hello"));
assert_eq!(ContentType::BINARY, inspect(b"\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00"));

assert!(inspect(b"Hello").is_text());
```
