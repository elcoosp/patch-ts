/// Detect the indentation style of a line.
/// Returns (uses_tabs, indent_level).
pub fn detect_indent(line: &str) -> (bool, usize) {
    let mut tab_count = 0;
    for ch in line.chars() {
        if ch == '\t' {
            tab_count += 1;
        } else {
            break;
        }
    }
    if tab_count > 0 {
        return (true, tab_count);
    }
    let space_count = line.chars().take_while(|c| *c == ' ').count();
    (false, space_count)
}

/// Apply the detected indentation style to each line of content.
pub fn apply_indent(content: &str, uses_tabs: bool, count: usize) -> String {
    if count == 0 {
        return content.to_string();
    }
    let indent_str = if uses_tabs {
        "\t".repeat(count)
    } else {
        " ".repeat(count)
    };
    content
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                line.to_string()
            } else {
                format!("{}{}", indent_str, line.trim())
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_indent_tabs() {
        let (uses, count) = detect_indent("\t\tlet x = 1;");
        assert!(uses);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_detect_indent_spaces() {
        let (uses, count) = detect_indent("    let x = 1;");
        assert!(!uses);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_apply_indent_tabs() {
        let new = apply_indent("let x = 2;\nlet y = 3;", true, 2);
        assert_eq!(new, "\t\tlet x = 2;\n\t\tlet y = 3;");
    }

    #[test]
    fn test_apply_indent_spaces() {
        let new = apply_indent("let x = 2;", false, 4);
        assert_eq!(new, "    let x = 2;");
    }

    #[test]
    fn test_empty_line_preserved() {
        let new = apply_indent("let x = 2;\n\nlet y = 3;", false, 2);
        assert_eq!(new, "  let x = 2;\n\n  let y = 3;");
    }
}
