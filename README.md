# content_inspector

[![Crates.io](https://img.shields.io/crates/v/content_inspector.svg)](https://crates.io/crates/content_inspector)
[![Documentation](https://docs.rs/content_inspector/badge.svg)](https://docs.rs/content_inspector)

A simple Rust crate for fast inspection of binary buffers to guess/determine the encoding.

``` rust
use content_inspector::{ContentType, inspect};

assert_eq!(ContentType::UTF_8, inspect(b"Hello"));
assert_eq!(ContentType::BINARY, inspect(b"\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00"));

assert!(inspect(b"Hello").is_text());
```
