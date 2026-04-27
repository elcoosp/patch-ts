use anyhow::Result;
use std::path::Path;
use tree_sitter::{Query, StreamingIterator};

use crate::ast::Language;

/// Run a tree‑sitter query and return a list of (start_byte, end_byte) ranges for the first match.
pub fn find_nodes_by_query(content: &str, query_str: &str, language: &mut dyn Language) -> Result<Vec<(usize, usize)>> {
    let parse_result = language.parse(content);
    let root = parse_result.tree.root_node();

    // For now, use RustLanguage's parser. In a future version we'll downcast the language.
    let query = Query::new(&tree_sitter_rust::LANGUAGE.into(), query_str)
        .map_err(|e| anyhow::anyhow!("Invalid tree‑sitter query: {}", e))?;

    let mut cursor = tree_sitter::QueryCursor::new();
    let mut matches = cursor.matches(&query, root, content.as_bytes());

    let mut nodes = Vec::new();
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let node = capture.node;
            nodes.push((node.start_byte(), node.end_byte()));
        }
    }

    Ok(nodes)
}

/// Replace the node matching a tree‑sitter query with new content.
pub fn apply_node_replace(file_path: &Path, query_str: &str, new_content: &str, language: &mut dyn Language) -> Result<()> {
    let original = std::fs::read_to_string(file_path)?;
    let nodes = find_nodes_by_query(&original, query_str, language)?;
    if nodes.is_empty() {
        anyhow::bail!("No nodes matched the query: {}", query_str);
    }
    let (start, end) = nodes[0];
    let mut patched = original.clone();
    patched.replace_range(start..end, new_content);
    std::fs::write(file_path, patched)?;
    Ok(())
}

/// Delete the node matching a tree‑sitter query.
pub fn apply_node_delete(file_path: &Path, query_str: &str, language: &mut dyn Language) -> Result<()> {
    let original = std::fs::read_to_string(file_path)?;
    let nodes = find_nodes_by_query(&original, query_str, language)?;
    if nodes.is_empty() {
        anyhow::bail!("No nodes matched the query: {}", query_str);
    }
    let (start, end) = nodes[0];
    let mut patched = original.clone();
    patched.replace_range(start..end, "");
    std::fs::write(file_path, patched)?;
    Ok(())
}

/// Insert text before the node matching a tree‑sitter query.
pub fn apply_node_insert_before(file_path: &Path, query_str: &str, text: &str, language: &mut dyn Language) -> Result<()> {
    let original = std::fs::read_to_string(file_path)?;
    let nodes = find_nodes_by_query(&original, query_str, language)?;
    if nodes.is_empty() {
        anyhow::bail!("No nodes matched the query: {}", query_str);
    }
    let (start, _) = nodes[0];
    let mut patched = original.clone();
    patched.insert_str(start, text);
    std::fs::write(file_path, patched)?;
    Ok(())
}
