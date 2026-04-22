use line_index::{LineIndex, TextSize};
use miette::NamedSource;
use tree_sitter::{Node, Parser, Tree};

use crate::diagnostics::SyntaxErrorDiagnostic;

#[derive(Debug, Clone, PartialEq)]
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

    fn from_byte_range(start_byte: usize, end_byte: usize, index: &LineIndex) -> Self {
        let start = index.line_col(TextSize::from(start_byte as u32));
        let end = index.line_col(TextSize::from(end_byte as u32));
        Span {
            start_byte,
            end_byte,
            start_line: start.line as usize + 1,
            start_column: start.col as usize + 1,
            end_line: end.line as usize + 1,
            end_column: end.col as usize + 1,
        }
    }

    pub fn shift(&self, offset: isize) -> Self {
        Span {
            start_byte: (self.start_byte as isize + offset) as usize,
            end_byte: (self.end_byte as isize + offset) as usize,
            start_line: self.start_line,
            start_column: self.start_column,
            end_line: self.end_line,
            end_column: self.end_column,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DelimiterError {
    Extra { span: Span, delimiter: char },
    Missing { expected: char, insert_at: Span, parent_kind: Option<String> },
}

impl DelimiterError {
    pub fn span(&self) -> &Span {
        match self {
            DelimiterError::Extra { span, .. } => span,
            DelimiterError::Missing { insert_at, .. } => insert_at,
        }
    }
    pub fn span_mut(&mut self) -> &mut Span {
        match self {
            DelimiterError::Extra { span, .. } => span,
            DelimiterError::Missing { insert_at, .. } => insert_at,
        }
    }
    pub fn delta(&self) -> isize {
        match self {
            DelimiterError::Extra { span, .. } => -((span.end_byte - span.start_byte) as isize),
            DelimiterError::Missing { .. } => 1,
        }
    }
}

pub struct ParseResult {
    pub tree: Tree,
    pub source: String,
    pub index: LineIndex,
}

impl ParseResult {
    pub fn text(&self) -> &str { &self.source }
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
    fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError>;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn diagnostic_message(&self, error: &DelimiterError) -> String;
}

// ----------------------------------------------------------------------
// AST traversal for delimiter errors (language-agnostic)
// ----------------------------------------------------------------------
pub(crate) fn find_delimiter_errors_via_ast(root: Node, index: &LineIndex) -> Vec<DelimiterError> {
    let mut errors = Vec::new();
    let mut stack: Vec<(char, usize, Option<String>)> = Vec::new();
    traverse_for_delimiters(root, &mut stack, &mut errors, index);
    for (expected, open_byte, parent_kind) in stack {
        errors.push(DelimiterError::Missing {
            expected,
            insert_at: Span::from_byte_range(open_byte, open_byte + 1, index),
            parent_kind,
        });
    }
    errors
}

fn traverse_for_delimiters(
    node: Node,
    stack: &mut Vec<(char, usize, Option<String>)>,
    errors: &mut Vec<DelimiterError>,
    index: &LineIndex,
) {
    let kind = node.kind();
    if kind.contains("string") || kind.contains("comment") || kind == "string_literal" || kind == "raw_string_literal" {
        return;
    }
    match kind {
        "(" | "[" | "{" => {
            let close = match kind {
                "(" => ')', "[" => ']', "{" => '}', _ => unreachable!(),
            };
            let parent_kind = node.parent().map(|p| p.kind().to_string());
            stack.push((close, node.start_byte(), parent_kind));
        }
        ")" | "]" | "}" => {
            let close_char = kind.chars().next().unwrap();
            if let Some((expected, open_byte, _)) = stack.pop() {
                if expected != close_char {
                    errors.push(DelimiterError::Extra {
                        span: Span::from_node(node, index),
                        delimiter: close_char,
                    });
                    stack.push((expected, open_byte, None));
                }
            } else {
                errors.push(DelimiterError::Extra {
                    span: Span::from_node(node, index),
                    delimiter: close_char,
                });
            }
        }
        _ => {}
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        traverse_for_delimiters(child, stack, errors, index);
    }
}

fn has_error_node(node: Node) -> bool {
    if node.is_error() { return true; }
    for child in node.children(&mut node.walk()) {
        if has_error_node(child) { return true; }
    }
    false
}

// ----------------------------------------------------------------------
// Language Implementations (via macro)
// ----------------------------------------------------------------------

macro_rules! impl_language {
    ($name:ident, $lang:expr) => {
        pub struct $name { parser: Parser }
        impl $name {
            pub fn new() -> Self {
                let mut parser = Parser::new();
                parser.set_language(&$lang.into()).unwrap();
                Self { parser }
            }
        }
        impl Language for $name {
            fn parse(&mut self, source: &str) -> ParseResult {
                let tree = self.parser.parse(source, None).unwrap();
                let index = LineIndex::new(source);
                ParseResult { tree, source: source.to_string(), index }
            }
            fn is_valid(&self, result: &ParseResult) -> bool { !has_error_node(result.tree.root_node()) }
            fn find_extra_delimiter(&self, _: &ParseResult) -> Option<Span> { None }
            fn explain_error(&self, result: &ParseResult, line: usize) -> Option<SyntaxErrorDiagnostic> {
                let node = result.node_at_line(line)?;
                if node.is_error() || node.has_error() {
                    let text = node.utf8_text(result.text().as_bytes()).unwrap_or("");
                    let details = format!("Syntax error near '{}'", text);
                    let span = Span::from_node(node, &result.index);
                    return Some(SyntaxErrorDiagnostic { src: NamedSource::new("input", result.text().to_string()), error_span: (span.start_byte, span.end_byte - span.start_byte).into(), details });
                }
                None
            }
            fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError> {
                find_delimiter_errors_via_ast(result.tree.root_node(), &result.index)
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
            fn diagnostic_message(&self, error: &DelimiterError) -> String {
                match error {
                    DelimiterError::Extra { delimiter, .. } => format!("Extra '{}'", delimiter),
                    DelimiterError::Missing { expected, .. } => format!("Missing '{}'", expected),
                }
            }
        }
    };
}

impl_language!(RustLanguage, tree_sitter_rust::LANGUAGE);
impl_language!(TypeScriptLanguage, tree_sitter_typescript::LANGUAGE_TYPESCRIPT);
impl_language!(JavaScriptLanguage, tree_sitter_javascript::LANGUAGE);
impl_language!(PythonLanguage, tree_sitter_python::LANGUAGE);
impl_language!(GoLanguage, tree_sitter_go::LANGUAGE);
impl_language!(RubyLanguage, tree_sitter_ruby::LANGUAGE);
impl_language!(PHPLanguage, tree_sitter_php::LANGUAGE_PHP);
impl_language!(HtmlLanguage, tree_sitter_html::LANGUAGE);
impl_language!(XmlLanguage, tree_sitter_xml::LANGUAGE_XML);
impl_language!(CLanguage, tree_sitter_c::LANGUAGE);
impl_language!(CppLanguage, tree_sitter_cpp::LANGUAGE);
impl_language!(JavaLanguage, tree_sitter_java::LANGUAGE);
impl_language!(CSharpLanguage, tree_sitter_c_sharp::LANGUAGE);

// Rust-specific extra methods
impl RustLanguage {
    pub fn find_function_body_range(&self, source: &str, function_name: &str) -> Option<(usize, usize)> {
        let pattern = format!("fn {}(", function_name);
        let mut start = 0;
        let mut found_range = None;
        while let Some(pos) = source[start..].find(&pattern) {
            let abs_pos = start + pos;
            let after_sig = &source[abs_pos..];
            let open_brace_offset = after_sig.find('{')?;
            let open_byte = abs_pos + open_brace_offset;
            let mut stack = 1;
            let mut close_byte = open_byte + 1;
            let chars = after_sig[open_brace_offset + 1..].chars();
            let mut in_string = false; let mut in_char = false; let mut escape = false;
            for c in chars {
                let char_len = c.len_utf8();
                if !in_string && !in_char {
                    if c == '"' { in_string = true; }
                    else if c == '\'' { in_char = true; }
                    else if c == '{' { stack += 1; }
                    else if c == '}' { stack -= 1; if stack == 0 { break; } }
                } else {
                    if escape { escape = false; }
                    else if c == '\\' { escape = true; }
                    else if (in_string && c == '"') || (in_char && c == '\'') { in_string = false; in_char = false; }
                }
                close_byte += char_len;
            }
            let range = (open_byte, close_byte);
            if found_range.is_some() { return None; }
            found_range = Some(range);
            start = abs_pos + 1;
        }
        found_range
    }
}
