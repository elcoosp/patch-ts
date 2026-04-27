use anyhow::Result;
use std::collections::HashMap;
use crate::ast::{CommentAnchor, Language, ParseResult};

#[derive(Debug, Clone)]
struct AnchoredComment {
    text: String,
    original_start: usize,
    anchor: CommentAnchor,
    is_doc: bool,
    is_inline: bool,
}

pub fn preserve_comments(
    original: &str,
    patched: &str,
    lang: &mut dyn Language,
) -> Result<String> {
    if original == patched { return Ok(patched.to_string()); }
    let orig_parse = lang.parse(original);
    let patched_parse = lang.parse(patched);
    let mut raw = lang.extract_comments(&orig_parse);
    lang.resolve_comment_anchors(&orig_parse, &mut raw);
    let anchored: Vec<AnchoredComment> = raw.into_iter()
        .filter_map(|c| c.anchor.map(|a| AnchoredComment {
            text: c.text, original_start: c.start_byte,
            anchor: a, is_doc: c.is_doc, is_inline: c.is_inline,
        }))
        .collect();
    if anchored.is_empty() { return Ok(patched.to_string()); }
    let entity_map = build_patched_entity_map(&patched_parse, lang);
    let mut repositioned: Vec<(usize, &AnchoredComment)> = anchored.iter()
        .filter_map(|ac| relocate_anchor(ac, &entity_map, patched).map(|pos| (pos, ac)))
        .collect();
    repositioned.sort_by(|a, b| b.0.cmp(&a.0));
    let mut result = patched.to_string();
    for (pos, ac) in &repositioned { insert_comment_at(&mut result, *pos, ac, patched); }
    Ok(result)
}

#[derive(Debug, Clone)]
struct PatchedEntity { start_byte: usize, end_byte: usize }

fn build_patched_entity_map(
    parse: &ParseResult, _lang: &mut dyn Language,
) -> HashMap<(String, Option<String>), Vec<PatchedEntity>> {
    let kinds = ["function_item","function_declaration","function_definition",
        "method_definition","struct_item","enum_item","trait_item","impl_item",
        "class_declaration","class_definition"];
    let mut map = HashMap::new();
    collect_patched_entities(parse.tree.root_node(), parse.text(), &kinds, &mut map);
    map
}

fn collect_patched_entities(
    node: tree_sitter::Node, source: &str,
    entity_kinds: &[&str], map: &mut HashMap<(String, Option<String>), Vec<PatchedEntity>>,
) {
    let k = node.kind();
    if entity_kinds.contains(&k) {
        let name = node.child_by_field_name("name")
            .and_then(|n| n.utf8_text(source.as_bytes()).ok())
            .map(|s| s.to_string());
        map.entry((k.to_string(), name)).or_default().push(PatchedEntity {
            start_byte: node.start_byte(), end_byte: node.end_byte(),
        });
    }
    for i in 0..node.child_count() {
        if let Some(c) = node.child(i as u32) { collect_patched_entities(c, source, entity_kinds, map); }
    }
}

fn relocate_anchor(
    ac: &AnchoredComment,
    entity_map: &HashMap<(String, Option<String>), Vec<PatchedEntity>>,
    patched: &str,
) -> Option<usize> {
    let key = (ac.anchor.entity_kind.clone(), ac.anchor.entity_name.clone());
    let entities = entity_map.get(&key)?;
    let entity = if entities.len() == 1 { &entities[0] } else { entities.first()? };
    let new_pos = (entity.start_byte as isize + ac.anchor.relative_offset) as usize;
    Some(new_pos.min(patched.len()))
}

fn insert_comment_at(content: &mut String, pos: usize, ac: &AnchoredComment, patched: &str) {
    let indent = {
        if pos == 0 || pos >= content.len() { String::new() }
        else {
            let sol = content[..pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
            content[sol..pos].chars().take_while(|c| c.is_whitespace()).collect()
        }
    };
    let text = if ac.is_doc && ac.anchor.relative_offset <= 0 {
        let mut adj = String::new();
        for line in ac.text.lines() {
            let t = line.trim();
            if t.is_empty() { adj.push('\n'); }
            else { adj.push_str(&format!("{}{}\n", indent, t)); }
        }
        adj
    } else if ac.is_inline {
        let line_end = patched[pos..].find('\n').map(|i| pos + i).unwrap_or(patched.len());
        let existing = patched[pos..line_end].trim_end();
        format!("{}{}", if existing.is_empty() { "" } else { " " }, ac.text.trim())
    } else {
        format!("{}{}\n", indent, ac.text.trim())
    };
    content.insert_str(pos, &text);
}
