use line_index::{LineIndex, TextSize};
use miette::NamedSource;
use tree_sitter::{Node, Parser, Tree};

use crate::diagnostics::SyntaxErrorDiagnostic;

#[derive(Debug, Clone)]
pub struct Span {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

impl Span {
    fn from_node(node: Node, index: &LineIndex) -> Self {
        let start = index.line_col(TextSize::from(node.start_byte() as u32));
        let end = index.line_col(TextSize::from(node.end_byte() as u32));
        Span {
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            start_line: start.line as usize + 1,
            start_column: start.col as usize + 1,
            end_line: end.line as usize + 1,
            end_column: end.col as usize + 1,
        }
    }
}

pub struct ParseResult {
    pub tree: Tree,
    pub source: String,
    index: LineIndex,
}

impl ParseResult {
    pub fn text(&self) -> &str {
        &self.source
    }

    pub fn node_at_line(&self, line: usize) -> Option<Node<'_>> {
        let root = self.tree.root_node();
        find_node_at_line(root, line, &self.index)
    }
}

fn find_node_at_line<'a>(node: Node<'a>, target_line: usize, index: &LineIndex) -> Option<Node<'a>> {
    let start_byte = node.start_byte();
    let start_pos = index.line_col(TextSize::from(start_byte as u32));
    if start_pos.line as usize + 1 == target_line {
        return Some(node);
    }
    for child in node.children(&mut node.walk()) {
        if let Some(found) = find_node_at_line(child, target_line, index) {
            return Some(found);
        }
    }
    None
}

pub trait Language {
    fn parse(&mut self, source: &str) -> ParseResult;
    fn is_valid(&self, result: &ParseResult) -> bool;
    fn find_extra_delimiter(&self, result: &ParseResult) -> Option<Span>;
    fn explain_error(&self, result: &ParseResult, line: usize) -> Option<SyntaxErrorDiagnostic>;
}

pub struct RustLanguage {
    parser: Parser,
}

impl RustLanguage {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        let language = tree_sitter_rust::LANGUAGE;
        parser
            .set_language(&language.into())
            .expect("failed to load Rust grammar");
        Self { parser }
    }

    fn has_error_node(&self, node: Node) -> bool {
        if node.is_error() {
            return true;
        }
        for child in node.children(&mut node.walk()) {
            if self.has_error_node(child) {
                return true;
            }
        }
        false
    }

    fn find_extra_in_node(&self, node: Node, index: &LineIndex, source: &str) -> Option<Span> {
        if node.is_error() {
            let text = node.utf8_text(source.as_bytes()).unwrap_or("");
            if text.contains('}') && node.parent().map_or(false, |p| p.kind() == "source_file") {
                return Some(Span::from_node(node, index));
            }
        }
        for child in node.children(&mut node.walk()) {
            if let Some(span) = self.find_extra_in_node(child, index, source) {
                return Some(span);
            }
        }
        None
    }
}

impl Language for RustLanguage {
    fn parse(&mut self, source: &str) -> ParseResult {
        let tree = self.parser.parse(source, None).unwrap();
        let index = LineIndex::new(source);
        ParseResult {
            tree,
            source: source.to_string(),
            index,
        }
    }

    fn is_valid(&self, result: &ParseResult) -> bool {
        !self.has_error_node(result.tree.root_node())
    }

    fn find_extra_delimiter(&self, result: &ParseResult) -> Option<Span> {
        let root = result.tree.root_node();
        self.find_extra_in_node(root, &result.index, result.text())
    }

    fn explain_error(&self, result: &ParseResult, line: usize) -> Option<SyntaxErrorDiagnostic> {
        let node = result.node_at_line(line)?;
        if node.is_error() {
            let text = node.utf8_text(result.text().as_bytes()).unwrap_or("");
            let details = if text.contains('}') {
                "Extra closing brace detected. Likely an extra '}' was inserted earlier.".to_string()
            } else {
                format!("Syntax error: unexpected '{}'", text)
            };
            let span = Span::from_node(node, &result.index);
            return Some(SyntaxErrorDiagnostic {
                src: NamedSource::new("input", result.text().to_string()),
                error_span: (span.start_byte, span.end_byte - span.start_byte).into(),
                details,
            });
        }
        None
    }
}

impl RustLanguage {
    /// Find the byte range of a function body by name using `syn` for robust parsing.
    /// Returns (start_byte, end_byte) if found and unique.
    pub fn find_function_body_range(&self, source: &str, function_name: &str) -> Option<(usize, usize)> {
        use syn::visit::Visit;
        use syn::File;

        struct FindFn<'a> {
            target: &'a str,
            range: Option<(usize, usize)>,
        }

        impl<'ast> Visit<'ast> for FindFn<'_> {
            fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
                if f.sig.ident == self.target {
                    if self.range.is_some() {
                        // Ambiguous
                        self.range = Some((0, 0));
                    } else {
                        let span = f.block.span();
                        let start = span.start().byte;
                        let end = span.end().byte;
                        self.range = Some((start, end));
                    }
                }
                // Continue visiting nested items
                syn::visit::visit_item_fn(self, f);
            }
        }

        let file = File::parse(source).ok()?;
        let mut visitor = FindFn { target: function_name, range: None };
        visitor.visit_file(&file);

        match visitor.range {
            Some((0, 0)) => None, // ambiguous
            Some(range) => Some(range),
            None => None,
        }
    }
}
