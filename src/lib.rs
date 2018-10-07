//! A simple library for *fast* inspection of binary buffers to guess the type of content.
//!
//! This is mainly intended to quickly determine whether a given buffer contains "binary"
//! or "text" data. Programs like `grep` or `git diff` use similar mechanisms to decide whether
//! to treat some files as "binary data" or not.
//!
//! The analysis is based on a very simple heuristic: Searching for NULL bytes
//! (indicating "binary" content) and the detection of special [byte order
//! marks](https://en.wikipedia.org/wiki/Byte_order_mark) (indicating a particular kind of textual
//! encoding). Note that **this analysis can fail**. For example, even if unlikely, UTF-8-encoded
//! text can legally contain NULL bytes. Conversely, some particular binary formats (like binary
//! [PGM](https://en.wikipedia.org/wiki/Netpbm_format)) may not contain NULL bytes. Also, for
//! performance reasons, only the first 1024 bytes are checked for the NULL-byte (if no BOM was
//! detected).
//!
//! If this library reports a certain type of encoding (say `UTF_16LE`), there is **no guarantee**
//! that the binary buffer can *actually* be decoded as UTF-16LE.
//!
//! # Example
//! ```
//! use content_inspector::{ContentType, inspect};
//!
//! assert_eq!(ContentType::UTF_8, inspect(b"Hello"));
//! assert_eq!(ContentType::BINARY, inspect(b"\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00"));
//!
//! assert!(inspect(b"Hello").is_text());
//! ```

extern crate memchr;

use memchr::memchr;
use std::cmp::min;
use std::fmt;

const MAX_SCAN_SIZE: usize = 1024;

/// The type of encoding that was detected (for "text" data) or `BINARY` for "binary" data.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ContentType {
    /// "binary" data
    BINARY,

    /// UTF-8 encoded "text" data
    UTF_8,

    /// UTF-8 encoded "text" data with a byte order mark.
    UTF_8_BOM,

    /// UTF-16 encoded "text" data (little endian)
    UTF_16LE,

    /// UTF-16 encoded "text" data (big endian)
    UTF_16BE,

    /// UTF-32 encoded "text" data (little endian)
    UTF_32LE,

    /// UTF-32 encoded "text" data (big endian)
    UTF_32BE,
}

impl ContentType {
    /// Returns `true`, if the `ContentType` is `BINARY`.
    pub fn is_binary(self) -> bool {
        self == ContentType::BINARY
    }

    /// Returns `true`, if the `ContentType` is __not__ `BINARY`.
    pub fn is_text(self) -> bool {
        !self.is_binary()
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ContentType::*;

        let name: &str = match *self {
            BINARY => "binary",
            UTF_8 => "UTF-8",
            UTF_8_BOM => "UTF-8-BOM",
            UTF_16LE => "UTF-16LE",
            UTF_16BE => "UTF-16BE",
            UTF_32LE => "UTF-32LE",
            UTF_32BE => "UTF-32BE",
        };
        write!(f, "{}", name)
    }
}

/// Common byte order marks
/// (see https://en.wikipedia.org/wiki/Byte_order_mark)
static BYTE_ORDER_MARKS: &[(&[u8], ContentType)] = &[
    (&[0xEF, 0xBB, 0xBF], ContentType::UTF_8_BOM),
    // UTF-32 needs to be checked before UTF-16 (overlapping BOMs)
    (&[0x00, 0x00, 0xFE, 0xFF], ContentType::UTF_32BE),
    (&[0xFF, 0xFE, 0x00, 0x00], ContentType::UTF_32LE),
    (&[0xFE, 0xFF], ContentType::UTF_16BE),
    (&[0xFF, 0xFE], ContentType::UTF_16LE),
];

/// Magic numbers for some filetypes that could otherwise be characterized as text.
static MAGIC_NUMBERS: [&[u8]; 2] = [b"%PDF", b"\x89PNG"];

/// Try to determine the type of content in the given buffer. See the crate documentation for a
/// usage example and for more details on how this analysis is performed.
///
/// If the buffer is empty, the content type will be reported as `UTF_8`.
pub fn inspect(buffer: &[u8]) -> ContentType {
    use ContentType::*;

    for &(bom, content_type) in BYTE_ORDER_MARKS {
        if buffer.starts_with(bom) {
            return content_type;
        }
    }

    // Scan the first few bytes for zero-bytes
    let scan_size = min(buffer.len(), MAX_SCAN_SIZE);
    let has_zero_bytes = memchr(0x00, &buffer[..scan_size]).is_some();

    if has_zero_bytes {
        return BINARY;
    }

    if MAGIC_NUMBERS.iter().any(|magic| buffer.starts_with(magic)) {
        return BINARY;
    }

    UTF_8
}

#[cfg(test)]
mod tests {
    use {inspect, ContentType::*};

    #[test]
    fn test_empty_buffer_utf_8() {
        assert_eq!(UTF_8, inspect(b""));
    }

    #[test]
    fn test_text_simple() {
        assert_eq!(UTF_8, inspect("Simple UTF-8 string ☔".as_bytes()));
    }

    #[test]
    fn test_text_utf8() {
        assert_eq!(UTF_8, inspect(include_bytes!("../testdata/text_UTF-8.txt")));
    }

    #[test]
    fn test_text_utf8_bom() {
        assert_eq!(
            UTF_8_BOM,
            inspect(include_bytes!("../testdata/text_UTF-8-BOM.txt"))
        );
    }

    #[test]
    fn test_text_utf16le() {
        assert_eq!(
            UTF_16LE,
            inspect(include_bytes!("../testdata/text_UTF-16LE-BOM.txt"))
        );
    }

    #[test]
    fn test_text_utf16be() {
        assert_eq!(
            UTF_16BE,
            inspect(include_bytes!("../testdata/text_UTF-16BE-BOM.txt"))
        );
    }

    #[test]
    fn test_text_utf32le() {
        assert_eq!(
            UTF_32LE,
            inspect(include_bytes!("../testdata/text_UTF-32LE-BOM.txt"))
        );
    }

    #[test]
    fn test_text_utf32be() {
        assert_eq!(
            UTF_32BE,
            inspect(include_bytes!("../testdata/text_UTF-32BE-BOM.txt"))
        );
    }

    #[test]
    fn test_png() {
        assert_eq!(BINARY, inspect(include_bytes!("../testdata/test.png")));
    }

    #[test]
    fn test_jpg() {
        assert_eq!(BINARY, inspect(include_bytes!("../testdata/test.jpg")));
    }

    #[test]
    fn test_pdf() {
        assert_eq!(BINARY, inspect(include_bytes!("../testdata/test.pdf")));
    }

    #[test]
    fn test_is_text() {
        assert!(UTF_8.is_text());
        assert!(UTF_32LE.is_text());
    }

    #[test]
    fn test_is_binary() {
        assert!(BINARY.is_binary());
    }
}
