/// Replace a byte range within a String with new text.
/// The range is in byte offsets, not character offsets.
///
/// # Panics
/// Panics if `range` is not within the bounds of `content` or not on
/// valid UTF‑8 boundaries (same as `String::replace_range`).
pub fn replace_byte_range(content: &mut String, range: std::ops::Range<usize>, new_text: &str) {
    content.replace_range(range, new_text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_single_line() {
        let mut content = "fn main() {}".to_string();
        // "main" is bytes 3..7
        replace_byte_range(&mut content, 3..7, "MAIN");
        assert_eq!(content, "fn MAIN() {}");
    }

    #[test]
    fn replace_across_lines_preserves_newlines() {
        let mut content = "line1\nline2\nline3".to_string();
        replace_byte_range(&mut content, 6..11, "REPLACED");
        assert_eq!(content, "line1\nREPLACED\nline3");
    }

    #[test]
    fn replace_empty_range_inserts() {
        let mut content = "hello".to_string();
        replace_byte_range(&mut content, 5..5, " world");
        assert_eq!(content, "hello world");
    }
}
