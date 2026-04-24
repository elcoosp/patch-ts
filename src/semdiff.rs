use serde::Serialize;
use std::collections::HashMap;
use tree_sitter::Parser;

#[derive(Debug, Clone, Serialize)]
pub struct Entity {
    pub name: String,
    pub kind: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum ChangeType {
    Added,
    Removed,
    Modified,
    Moved,
}

#[derive(Debug, Clone, Serialize)]
pub struct EntityChange {
    pub change_type: ChangeType,
    pub entity: Entity,
    pub old_entity: Option<Entity>,
    pub diff: Option<String>,
}

/// Extract entities (functions, structs, classes, etc.) from source code.
pub fn extract_entities(source: &str, lang: &str) -> Result<Vec<Entity>, String> {
    let mut parser = Parser::new();
    let language = match lang {
        "rs" => tree_sitter_rust::LANGUAGE.into(),
        "ts" | "tsx" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "js" | "jsx" => tree_sitter_javascript::LANGUAGE.into(),
        "py" | "pyi" => tree_sitter_python::LANGUAGE.into(),
        "go" => tree_sitter_go::LANGUAGE.into(),
        "rb" => tree_sitter_ruby::LANGUAGE.into(),
        "php" => tree_sitter_php::LANGUAGE_PHP.into(),
        "html" => tree_sitter_html::LANGUAGE.into(),
        "xml" => tree_sitter_xml::LANGUAGE_XML.into(),
        "c" => tree_sitter_c::LANGUAGE.into(),
        "cpp" => tree_sitter_cpp::LANGUAGE.into(),
        "java" => tree_sitter_java::LANGUAGE.into(),
        "csharp" => tree_sitter_c_sharp::LANGUAGE.into(),
        "swift" => tree_sitter_swift::LANGUAGE.into(),
        "scala" => tree_sitter_scala::LANGUAGE.into(),
        "zig" => tree_sitter_zig::LANGUAGE.into(),
        _ => return Err(format!("Unsupported language: {}", lang)),
    };
    parser.set_language(&language).map_err(|e| e.to_string())?;
    let tree = parser.parse(source, None).ok_or("Failed to parse")?;
    let root = tree.root_node();

    let entity_kinds: Vec<&str> = vec![
        "function_item",
        "struct_item",
        "enum_item",
        "impl_item",
        "trait_item",
        "function_declaration",
        "class_declaration",
        "method_definition",
        "function_definition",
        "class_definition",
    ];

    let mut entities = Vec::new();
    collect_entities(&root, source, &entity_kinds, &mut entities);
    Ok(entities)
}

fn collect_entities(
    node: &tree_sitter::Node,
    source: &str,
    entity_kinds: &[&str],
    entities: &mut Vec<Entity>,
) {
    let kind = node.kind();
    if entity_kinds.contains(&kind) {
        if let Some(name_node) = node.child_by_field_name("name") {
            if let Ok(name) = name_node.utf8_text(source.as_bytes()) {
                // Build a simple signature from the node's text up to the body
                let sig = node.utf8_text(source.as_bytes()).unwrap_or("");
                let sig_trimmed = sig.lines().next().unwrap_or(sig).to_string();
                entities.push(Entity {
                    name: name.to_string(),
                    kind: kind.to_string(),
                    start_byte: node.start_byte(),
                    end_byte: node.end_byte(),
                    signature: sig_trimmed,
                });
            }
        }
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i as u32) {
            collect_entities(&child, source, entity_kinds, entities);
        }
    }
}

/// Diff two lists of entities and return a list of changes.
pub fn diff_entities(old: &[Entity], new: &[Entity]) -> Vec<EntityChange> {
    let mut changes = Vec::new();
    let old_map: HashMap<&str, &Entity> = old.iter().map(|e| (e.name.as_str(), e)).collect();
    let new_map: HashMap<&str, &Entity> = new.iter().map(|e| (e.name.as_str(), e)).collect();

    // Added
    for (name, entity) in &new_map {
        if !old_map.contains_key(name) {
            changes.push(EntityChange {
                change_type: ChangeType::Added,
                entity: (*entity).clone(),
                old_entity: None,
                diff: None,
            });
        }
    }
    // Removed
    for (name, entity) in &old_map {
        if !new_map.contains_key(name) {
            changes.push(EntityChange {
                change_type: ChangeType::Removed,
                entity: (*entity).clone(),
                old_entity: None,
                diff: None,
            });
        }
    }
    // Modified (signature differs)
    for (name, new_entity) in &new_map {
        if let Some(old_entity) = old_map.get(name) {
            if new_entity.signature != old_entity.signature
                || new_entity.start_byte != old_entity.start_byte
            {
                changes.push(EntityChange {
                    change_type: ChangeType::Modified,
                    entity: (*new_entity).clone(),
                    old_entity: Some((*old_entity).clone()),
                    diff: None,
                });
            }
        }
    }
    // Moved (same name, same sig, different location) – already covered by Modified for now
    changes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rust_entities() {
        let source = "fn foo() {}\nfn bar() {}\nstruct Baz {}";
        let entities = extract_entities(source, "rs").unwrap();
        let names: Vec<&str> = entities.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"bar"));
        assert!(names.contains(&"Baz"));
    }

    #[test]
    fn test_diff_added_removed() {
        let old = vec![Entity {
            name: "foo".into(),
            kind: "function_item".into(),
            start_byte: 0,
            end_byte: 10,
            signature: "fn foo()".into(),
        }];
        let new = vec![Entity {
            name: "bar".into(),
            kind: "function_item".into(),
            start_byte: 0,
            end_byte: 10,
            signature: "fn bar()".into(),
        }];
        let changes = diff_entities(&old, &new);
        assert_eq!(changes.len(), 2);
        assert!(changes
            .iter()
            .any(|c| matches!(c.change_type, ChangeType::Added)));
        assert!(changes
            .iter()
            .any(|c| matches!(c.change_type, ChangeType::Removed)));
    }
}
