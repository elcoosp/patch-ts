use std::collections::HashSet;
use tree_sitter::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

/// Extract all identifiers from source code using tree-sitter.
pub fn extract_identifiers(source: &str, language: &str) -> Result<HashSet<String>, String> {
    let mut parser = Parser::new();
    let lang = match language {
        "rs" => tree_sitter_rust::LANGUAGE.into(),
        _ => {
            return Err(format!(
                "Identifier extraction not supported for {}",
                language
            ))
        }
    };
    parser.set_language(&lang).map_err(|e| e.to_string())?;
    let tree = parser.parse(source, None).ok_or("Failed to parse")?;
    let root = tree.root_node();

    let query_str = "(identifier) @id";
    let query = Query::new(&lang, query_str).map_err(|e| e.to_string())?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root, source.as_bytes());

    let mut ids: HashSet<String> = HashSet::new();
    while let Some(match_item) = matches.next() {
        for capture in match_item.captures {
            if let Ok(text) = capture.node.utf8_text(source.as_bytes()) {
                ids.insert(text.to_string() as String);
            }
        }
    }
    Ok(ids)
}

/// Check which identifiers in `new_text` do not appear in `original_source`.
pub fn missing_identifiers(original_source: &str, new_text: &str, language: &str) -> Vec<String> {
    let orig_ids = match extract_identifiers(original_source, language) {
        Ok(ids) => ids,
        Err(_) => return vec![],
    };
    let new_ids = match extract_identifiers(new_text, language) {
        Ok(ids) => ids,
        Err(_) => return vec![],
    };
    new_ids.difference(&orig_ids).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_identifiers() {
        let source = "fn main() { let x = 1; let y = 2; }";
        let ids = extract_identifiers(source, "rs").unwrap();
        assert!(ids.contains("main"));
        assert!(ids.contains("x"));
        assert!(ids.contains("y"));
    }

    #[test]
    fn test_missing_identifiers() {
        let source = "fn main() { let x = 1; }";
        let new_text = "let y = 2;";
        let missing = missing_identifiers(source, new_text, "rs");
        assert!(missing.contains(&"y".to_string()));
    }
}
