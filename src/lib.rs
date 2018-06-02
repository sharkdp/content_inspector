extern crate memchr;

use memchr::memchr;
use std::cmp::min;

const MAX_SCAN_SIZE: usize = 1024;

// Common byte order marks
// See: https://en.wikipedia.org/wiki/Byte_order_mark
static BYTE_ORDER_MARKS: &[&[u8]] = &[
    &[0xEF, 0xBB, 0xBF],       // UTF-8
    &[0xFE, 0xFF],             // UTF-16 (BE)
    &[0xFF, 0xFE],             // UTF-16 (LE)
    &[0x00, 0x00, 0xFE, 0xFF], // UTF-32 (BE)
    &[0xFF, 0xFE, 0x00, 0x00], // UTF-32 (LE)
];

static MAGIC_NUMBER_PDF: &[u8] = b"%PDF";

/// Try to determine whether the bytes in the buffer are printable text or "binary data".
///
/// ```rust
/// use text_or_binary::is_text;
///
/// assert!(is_text(b"Hello"));
/// assert!(!is_text(b"\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00"));
/// ```
pub fn is_text(buffer: &[u8]) -> bool {
    // Scan the first few bytes for zero-bytes
    let scan_size = min(buffer.len(), MAX_SCAN_SIZE);
    let has_zero_bytes = memchr(0x00, &buffer[..scan_size]).is_some();

    if has_zero_bytes {
        if BYTE_ORDER_MARKS.iter().any(|bom| buffer.starts_with(bom)) {
            return true;
        }

        return false;
    }

    if buffer.starts_with(MAGIC_NUMBER_PDF) {
        return false;
    }

    // TODO: are the any common binary formats without 0-bytes?
    return true;
}

/// The opposite of `is_text`.
pub fn is_binary(buffer: &[u8]) -> bool {
    if buffer.is_empty() {
        return true;
    }
    return !is_text(buffer);
}

#[cfg(test)]
mod tests {
    use {is_binary, is_text};

    #[test]
    fn test_is_text_empty() {
        assert!(is_text(b""));
    }

    #[test]
    fn test_is_text_utf8() {
        assert!(is_text(
            "simple text and\nsome characters like 🌂, 💖, ä, 𝄞, € and ∰".as_bytes()
        ));
    }

    // #[test]
    // fn test_is_text_utf8_with_null() {
    //     assert!(is_text(
    //         "simple text and a \x00 character".as_bytes()
    //     ));
    // }

    #[test]
    fn test_is_text_utf16le() {
        assert!(is_text(b"\xFE\xFF\x73\x00"));
    }

    #[test]
    fn test_is_text_utf16be() {
        assert!(is_text(b"\xFF\xFE\x00\x73"));
    }

    #[test]
    fn test_is_binary_empty() {
        assert!(is_binary(b""));
    }

    #[test]
    fn test_jpeg() {
        let jpeg_header = b"\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00";
        assert!(is_binary(jpeg_header));
    }

    #[test]
    fn test_is_binary_pdf() {
        let pdf_header = "%PDF-1.4\n";
        assert!(is_binary(pdf_header.as_bytes()));
    }
}
