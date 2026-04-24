use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};
use glob::glob;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub kind: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEdge {
    pub caller_file: String,
    pub caller_line: usize,
    pub callee: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub symbols: HashMap<String, SymbolInfo>,
    pub call_edges: Vec<CallEdge>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self { symbols: HashMap::new(), call_edges: Vec::new() }
    }

    pub fn callers_of(&self, symbol: &str) -> Vec<&CallEdge> {
        self.call_edges.iter().filter(|e| e.callee == symbol).collect()
    }
}

/// Build a project‑wide knowledge graph by indexing all source files.
pub fn build_project_index(root: &Path) -> KnowledgeGraph {
    let mut kg = KnowledgeGraph::new();
    let pattern = root.join("**").join("*.rs"); // Focus on Rust for now
    if let Some(pattern_str) = pattern.to_str() {
        for entry in glob(pattern_str).unwrap().flatten() {
            if let Ok(content) = std::fs::read_to_string(&entry) {
                if let Ok(entities) = crate::semdiff::extract_entities(&content, "rs") {
                    let file_path = entry.to_string_lossy().to_string();
                    for entity in entities {
                        let name = entity.name.clone();
                        kg.symbols.insert(name.clone(), SymbolInfo {
                            kind: entity.kind,
                            file: file_path.clone(),
                            line: entity.signature.lines().next().map(|l| l.len()).unwrap_or(0), // placeholder
                            column: 0,
                        });
                        // Build call edges: for each function call in the file, add an edge
                        // Simplified: use regex to find calls to this entity
                        let call_re = regex::Regex::new(&format!(r"\b{}\s*\(", regex::escape(&name))).unwrap();
                        for (i, line) in content.lines().enumerate() {
                            if call_re.is_match(line) {
                                kg.call_edges.push(CallEdge {
                                    caller_file: file_path.clone(),
                                    caller_line: i + 1,
                                    callee: name.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    kg
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_build_project_index() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(dir.path().join("src").join("lib.rs"), "fn foo() {}\nfn bar() { foo(); }").unwrap();

        let kg = build_project_index(dir.path());
        assert!(kg.symbols.contains_key("foo"));
        assert!(kg.symbols.contains_key("bar"));
        assert!(kg.call_edges.iter().any(|e| e.callee == "foo"));
    }
}
