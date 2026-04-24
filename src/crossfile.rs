use crate::symbols::SymbolIndex;
use std::collections::{HashMap, VecDeque};
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

#[derive(Debug, Clone)]
pub struct CallSite {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct CallEdge {
    pub caller_file: String,
    pub caller_line: usize,
    pub caller_column: usize,
    pub callee: String,
}

#[derive(Debug, Default)]
pub struct CallGraph {
    pub edges: Vec<CallEdge>,
}

impl CallGraph {
    pub fn new() -> Self { Self { edges: vec![] } }

    pub fn add_edge(&mut self, caller_file: String, caller_line: usize, caller_column: usize, callee: String) {
        self.edges.push(CallEdge { caller_file, caller_line, caller_column, callee });
    }

    pub fn callers_of(&self, func_name: &str) -> Vec<&CallEdge> {
        self.edges.iter().filter(|e| e.callee == func_name).collect()
    }

    pub fn all_callers_recursive(&self, func_name: &str) -> Vec<&CallEdge> {
        let mut result = vec![];
        let mut visited = std::collections::HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(func_name.to_string());
        while let Some(current) = queue.pop_front() {
            for edge in self.edges.iter().filter(|e| e.callee == current) {
                let key = format!("{}:{}:{}", edge.caller_file, edge.caller_line, edge.caller_column);
                if visited.insert(key.clone()) {
                    result.push(edge);
                    queue.push_back(edge.callee.clone());
                }
            }
        }
        result
    }
}

pub fn build_project_call_graph(root: &Path) -> CallGraph {
    let mut graph = CallGraph::new();
    let pattern = root.join("**").join("*.rs");
    if let Some(pattern_str) = pattern.to_str() {
        for entry in glob::glob(pattern_str).unwrap().flatten() {
            if let Ok(content) = std::fs::read_to_string(&entry) {
                let file_path = entry.to_string_lossy().to_string();
                if let Ok(calls) = extract_calls(&content, "rs") {
                    for call in calls {
                        graph.add_edge(file_path.clone(), call.line, call.column, call.name);
                    }
                }
            }
        }
    }
    graph
}

#[derive(Debug, Clone)]
pub struct CallInfo {
    pub name: String,
    pub line: usize,
    pub column: usize,
}

pub fn extract_calls(source: &str, lang: &str) -> Result<Vec<CallInfo>, Box<dyn std::error::Error>> {
    let mut parser = Parser::new();
    let language = match lang {
        "rs" => tree_sitter_rust::LANGUAGE.into(),
        "ts" | "tsx" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "js" | "jsx" => tree_sitter_javascript::LANGUAGE.into(),
        _ => return Err("Unsupported language".into()),
    };
    parser.set_language(&language)?;
    let tree = parser.parse(source, None).ok_or("Failed to parse")?;
    let root = tree.root_node();

    let query_str = match lang {
        "rs" => "(call_expression function: (identifier) @func)",
        "ts" | "tsx" | "js" | "jsx" => "(call_expression function: (identifier) @func)",
        _ => return Err("Unsupported language for query".into()),
    };
    let query = Query::new(&language, query_str)?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root, source.as_bytes());
    let mut calls = vec![];
    while let Some(m) = matches.next() {
        for capture in m.captures {
            if let Ok(name) = capture.node.utf8_text(source.as_bytes()) {
                let pos = capture.node.start_position();
                calls.push(CallInfo {
                    name: name.to_string(),
                    line: pos.row + 1,
                    column: pos.column + 1,
                });
            }
        }
    }
    Ok(calls)
}

pub fn build_project_index(_root: &Path) -> HashMap<String, SymbolIndex> {
    HashMap::new()
}

pub fn find_callers(_func_name: &str, _index: &HashMap<String, SymbolIndex>) -> Vec<CallSite> {
    vec![]
}
