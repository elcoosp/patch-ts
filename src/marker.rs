use anyhow::Result;
use tree_sitter::{Node, Parser};

use crate::ast::{Language, RustLanguage};

#[derive(Debug)]
pub struct MarkerLocation {
    pub line: usize,
    pub byte_range: (usize, usize),
    pub node_to_replace: Option<Node<'static>>, // simplified for now
}

/// Find all markers with given ID in source.
pub fn find_marker(source: &str, marker_id: &str) -> Result<Vec<MarkerLocation>> {
    let _lang = RustLanguage::new();
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    let mut markers = Vec::new();
    let pattern = format!("// PATCH-ME: {}", marker_id);
    let block_pattern = format!("/* PATCH-ME: {} */", marker_id);

    find_markers_in_node(root, source, &pattern, &block_pattern, &mut markers);
    if markers.is_empty() {
        anyhow::bail!("Marker '{}' not found", marker_id);
    }
    Ok(markers)
}

fn find_markers_in_node(node: Node, source: &str, pattern: &str, block_pattern: &str, markers: &mut Vec<MarkerLocation>) {
    if node.kind() == "line_comment" || node.kind() == "block_comment" {
        if let Ok(text) = node.utf8_text(source.as_bytes()) {
            if text.contains(pattern) || text.contains(block_pattern) {
                let start = node.start_position();
                markers.push(MarkerLocation {
                    line: start.row + 1,
                    byte_range: (node.start_byte(), node.end_byte()),
                    node_to_replace: None,
                });
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        find_markers_in_node(child, source, pattern, block_pattern, markers);
    }
}

/// Replace the AST node associated with a marker.
pub fn replace_marker_node(source: &str, marker_id: &str, new_content: &str) -> Result<String> {
    let markers = find_marker(source, marker_id)?;
    if markers.len() > 1 {
        anyhow::bail!("Multiple markers with ID '{}' found", marker_id);
    }
    let marker = &markers[0];

    let mut lang = RustLanguage::new();
    let parse_result = lang.parse(source);
    let root = parse_result.tree.root_node();

    let marker_node = find_node_at_byte(root, marker.byte_range.0)?;
    let _cursor = marker_node.walk();
    let node_to_replace = marker_node.next_sibling().or_else(|| marker_node.parent().and_then(|p| p.next_sibling()));

    if let Some(node) = node_to_replace {
        let mut new_source = source.to_string();
        new_source.replace_range(node.start_byte()..node.end_byte(), new_content);
        Ok(new_source)
    } else {
        anyhow::bail!("No node found after marker to replace");
    }
}

fn find_node_at_byte(node: Node, byte: usize) -> Result<Node> {
    if node.start_byte() <= byte && node.end_byte() >= byte {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.start_byte() <= byte && child.end_byte() >= byte {
                return find_node_at_byte(child, byte);
            }
        }
        Ok(node)
    } else {
        anyhow::bail!("byte not in node")
    }
}
