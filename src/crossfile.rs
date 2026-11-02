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
    /// Name of the function containing the call site (empty at top level).
    /// This is what makes transitive caller analysis possible.
    pub caller_function: String,
    pub caller_line: usize,
    pub caller_column: usize,
    pub callee: String,
}

#[derive(Debug, Default)]
pub struct CallGraph {
    pub edges: Vec<CallEdge>,
}

impl CallGraph {
    #[must_use]
    pub fn new() -> Self {
        Self { edges: vec![] }
    }

    pub fn add_edge(
        &mut self,
        caller_file: String,
        caller_function: String,
        caller_line: usize,
        caller_column: usize,
        callee: String,
    ) {
        self.edges.push(CallEdge {
            caller_file,
            caller_function,
            caller_line,
            caller_column,
            callee,
        });
    }

    #[must_use]
    pub fn callers_of(&self, func_name: &str) -> Vec<&CallEdge> {
        self.edges
            .iter()
            .filter(|e| e.callee == func_name)
            .collect()
    }

    #[must_use]
    pub fn all_callers_recursive(&self, func_name: &str) -> Vec<&CallEdge> {
        let mut result = vec![];
        let mut visited = std::collections::HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(func_name.to_string());
        while let Some(current) = queue.pop_front() {
            for edge in self.edges.iter().filter(|e| e.callee == current) {
                let key = format!(
                    "{}:{}:{}:{}",
                    edge.caller_file, edge.caller_function, edge.caller_line, edge.caller_column
                );
                if visited.insert(key.clone()) {
                    result.push(edge);
                    // Walk up through the calling function so that transitive
                    // callers (A calls B calls C) are reported as well.
                    if !edge.caller_function.is_empty() {
                        queue.push_back(edge.caller_function.clone());
                    }
                }
            }
        }
        result
    }
}

#[must_use]
///
/// # Panics
///
/// Panics if an internal `unwrap` fails (invariant violation).
pub fn build_project_call_graph(root: &Path) -> CallGraph {
    let mut graph = CallGraph::new();
    let pattern = root.join("**").join("*.rs");
    if let Some(pattern_str) = pattern.to_str() {
        for entry in glob::glob(pattern_str).unwrap().flatten() {
            if let Ok(content) = std::fs::read_to_string(&entry) {
                let file_path = entry.to_string_lossy().to_string();
                if let Ok(calls) = extract_calls(&content, "rs") {
                    for call in calls {
                        graph.add_edge(
                            file_path.clone(),
                            call.caller,
                            call.line,
                            call.column,
                            call.name,
                        );
                    }
                }
            }
        }
    }
    graph
}

#[derive(Debug, Clone)]
pub struct CallInfo {
    /// Name of the callee.
    pub name: String,
    /// Name of the enclosing function (empty at top level).
    pub caller: String,
    pub line: usize,
    pub column: usize,
}

///
/// # Errors
///
/// Returns an error if the input cannot be parsed.
pub fn extract_calls(
    source: &str,
    lang: &str,
) -> Result<Vec<CallInfo>, Box<dyn std::error::Error>> {
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
        "rs" | "ts" | "tsx" | "js" | "jsx" => "(call_expression function: (identifier) @func)",
        _ => return Err("Unsupported language for query".into()),
    };
    let query = Query::new(&language, query_str)?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root, source.as_bytes());
    let mut calls = vec![];
    while let Some(m) = matches.next() {
        for capture in m.captures() {
            if let Ok(name) = capture.node.utf8_text(source.as_bytes()) {
                let pos = capture.node.start_position();
                // Find the enclosing function so the caller graph can chain
                // transitively (A -> B -> C).
                let mut caller = String::new();
                let mut ancestor = capture.node.parent();
                while let Some(node) = ancestor {
                    if node.kind() == "function_item"
                        && let Some(name_node) = node.child_by_field_name("name")
                        && let Ok(fn_name) = name_node.utf8_text(source.as_bytes())
                    {
                        caller = fn_name.to_string();
                        break;
                    }
                    ancestor = node.parent();
                }
                calls.push(CallInfo {
                    name: name.to_string(),
                    caller,
                    line: pos.row + 1,
                    column: pos.column + 1,
                });
            }
        }
    }
    Ok(calls)
}

#[must_use]
pub fn build_project_index(_root: &Path) -> HashMap<String, SymbolIndex> {
    HashMap::new()
}

#[must_use]
pub fn find_callers<S: ::std::hash::BuildHasher>(
    _func_name: &str,
    _index: &HashMap<String, SymbolIndex, S>,
) -> Vec<CallSite> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn call_graph_tracks_callers() {
        let mut graph = CallGraph::new();
        assert!(graph.edges.is_empty());
        graph.add_edge("a.rs".into(), "caller_a".into(), 3, 5, "helper".into());
        graph.add_edge("b.rs".into(), "caller_b".into(), 9, 1, "helper".into());
        assert_eq!(graph.callers_of("helper").len(), 2);
        assert!(graph.callers_of("missing").is_empty());
        assert_eq!(graph.all_callers_recursive("helper").len(), 2);
    }

    #[test]
    fn call_graph_recursive_callers_follow_chain() {
        let mut graph = CallGraph::new();
        // leaf <- middle <- top: transitive callers chain through the
        // enclosing function name.
        graph.add_edge("middle.rs".into(), "middle".into(), 2, 1, "leaf".into());
        graph.add_edge("top.rs".into(), "top".into(), 7, 2, "middle".into());
        assert_eq!(graph.all_callers_recursive("leaf").len(), 2);
        assert_eq!(graph.all_callers_recursive("middle").len(), 1);
    }

    #[test]
    fn call_graph_recursive_callers_handle_cycles() {
        let mut graph = CallGraph::new();
        graph.add_edge("a.rs".into(), "a_fn".into(), 1, 1, "b".into());
        graph.add_edge("b.rs".into(), "b_fn".into(), 1, 1, "a".into());
        // Must terminate despite the cycle.
        assert!(!graph.all_callers_recursive("a").is_empty());
    }

    #[test]
    fn extract_calls_finds_rust_call_sites() {
        let source = "fn caller() {\n    helper();\n    other(1);\n}\n";
        let calls = extract_calls(source, "rs").unwrap();
        let names: Vec<&str> = calls.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"helper"));
        assert!(names.contains(&"other"));
        assert_eq!(calls[0].line, 2);
        assert_eq!(calls[0].column, 5);
    }

    #[test]
    fn extract_calls_rejects_unsupported_language() {
        assert!(extract_calls("fn f() {}", "cobol").is_err());
    }

    #[test]
    fn build_project_call_graph_scans_workspace() {
        let dir = tempdir().unwrap();
        let lib_dir = dir.path().join("src");
        fs::create_dir_all(&lib_dir).unwrap();
        fs::write(
            lib_dir.join("lib_a.rs"),
            "pub fn helper() {}\nfn user() { helper(); }\n",
        )
        .unwrap();
        let graph = build_project_call_graph(dir.path());
        assert!(
            !graph.callers_of("helper").is_empty(),
            "expected at least one caller of helper"
        );
    }

    #[test]
    fn stub_builders_return_empty() {
        let dir = tempdir().unwrap();
        assert!(build_project_index(dir.path()).is_empty());
        let index = build_project_index(dir.path());
        assert!(find_callers("anything", &index).is_empty());
    }
}
