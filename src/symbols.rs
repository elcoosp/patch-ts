use crate::ast::Span;
use std::collections::HashMap;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

#[derive(Debug, Clone, serde::Serialize)]
pub struct SymbolIndex {
    pub functions: HashMap<String, Span>,
}

#[must_use]
pub fn build_index(source: &str, lang: &str) -> Option<SymbolIndex> {
    let mut parser = Parser::new();
    let language = match lang {
        "rs" => tree_sitter_rust::LANGUAGE.into(),
        "ts" | "tsx" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "js" | "jsx" => tree_sitter_javascript::LANGUAGE.into(),
        "py" | "pyi" => tree_sitter_python::LANGUAGE.into(),
        "go" => tree_sitter_go::LANGUAGE.into(),
        _ => return None,
    };
    parser.set_language(&language).ok()?;
    let tree = parser.parse(source, None)?;
    let root = tree.root_node();

    let query_str = match lang {
        "rs" | "ts" | "tsx" | "js" | "jsx" => "(function_item name: (identifier) @name) @item",
        "py" => "(function_definition name: (identifier) @name) @item",
        "go" => "(function_declaration name: (identifier) @name) @item",
        _ => return None,
    };
    let query = Query::new(&language, query_str).ok()?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root, source.as_bytes());

    let mut functions = HashMap::new();
    while let Some(m) = matches.next() {
        let capture = m
            .captures()
            .iter()
            .find(|c| c.node.kind() == "identifier")?;
        let name = capture.node.utf8_text(source.as_bytes()).ok()?.to_string();
        let span = Span {
            start_byte: capture.node.start_byte(),
            end_byte: capture.node.end_byte(),
            start_line: capture.node.start_position().row + 1,
            start_column: capture.node.start_position().column + 1,
            end_line: capture.node.end_position().row + 1,
            end_column: capture.node.end_position().column + 1,
        };
        functions.insert(name, span);
    }
    Some(SymbolIndex { functions })
}

impl SymbolIndex {
    #[must_use]
    pub fn lookup_function(&self, name: &str) -> Option<&Span> {
        self.functions.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_index_finds_rust_functions() {
        let source = "fn alpha() {}\nfn beta(x: u32) {}\n";
        let index = build_index(source, "rs").unwrap();
        assert!(index.lookup_function("alpha").is_some());
        assert!(index.lookup_function("beta").is_some());
        assert!(index.lookup_function("gamma").is_none());
        let span = index.lookup_function("alpha").unwrap();
        assert_eq!(span.start_line, 1);
    }

    #[test]
    fn build_index_supports_python() {
        let index = build_index("def run():\n    pass\n", "py").unwrap();
        assert!(index.lookup_function("run").is_some());
    }

    #[test]
    fn build_index_rejects_unknown_language() {
        assert!(build_index("fn f() {}", "cobol").is_none());
    }
}
