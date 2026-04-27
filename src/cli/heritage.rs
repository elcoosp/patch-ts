use anyhow::Result;

/// Parse a SEARCH/REPLACE block from heredoc input.
/// Format:
/// <<< SEARCH
/// <search block>
/// ---
/// <replace block>
pub fn parse_search_replace_block(input: &str) -> Result<(String, String)> {
    let input = input.trim();
    // Remove optional fence markers
    let content = if let Some((_, extracted)) = crate::sanitize::extract_fenced_block(input) {
        extracted.to_string()
    } else {
        input.to_string()
    };

    // Expect the block to start with <<< SEARCH (optional newline)
    let header = "<<< SEARCH";
    let body = if let Some(stripped) = content.strip_prefix(header) {
        stripped.trim_start()
    } else {
        anyhow::bail!("Input does not start with '<<< SEARCH' header");
    };

    // Split into search and replace at "\n---\n"
    let parts: Vec<&str> = body.splitn(2, "\n---\n").collect();
    if parts.len() != 2 {
        anyhow::bail!("SEARCH/REPLACE block must contain a '---' separator between search and replace sections");
    }

    let search = parts[0].trim().to_string();
    let replace = parts[1].trim().to_string();

    if search.is_empty() {
        anyhow::bail!("SEARCH block cannot be empty");
    }

    Ok((search, replace))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_block() {
        let input = "<<< SEARCH\nold content\n---\nnew content";
        let (search, replace) = parse_search_replace_block(input).unwrap();
        assert_eq!(search, "old content");
        assert_eq!(replace, "new content");
    }

    #[test]
    fn test_missing_separator() {
        let input = "<<< SEARCH\nold content only";
        assert!(parse_search_replace_block(input).is_err());
    }
}
