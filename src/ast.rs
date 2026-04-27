use line_index::{LineIndex, TextSize};
use miette::NamedSource;
use tree_sitter::{Node, Parser, Tree, StreamingIterator};

use crate::diagnostics::SyntaxErrorDiagnostic;

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
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
    Extra {
        span: Span,
        delimiter: char,
    },
    Missing {
        expected: char,
        insert_at: Span,
        parent_kind: Option<String>,
    },
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
    if start_pos.line as usize + 1 == target_line { return Some(node); }
    for child in node.children(&mut node.walk()) {
        if let Some(found) = find_node_at_line(child, target_line, index) { return Some(found); }
    }
    None
}

/// Represents a named code entity (function, struct, class, etc.)
#[derive(Debug, Clone, serde::Serialize)]
pub struct Entity {
    pub name: String,
    pub kind: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub signature: String,
}

/// Anchor describing which entity a comment belongs to
#[derive(Debug, Clone, serde::Serialize)]
pub struct CommentAnchor {
    pub entity_kind: String,
    pub entity_name: Option<String>,
    pub relative_offset: isize,
}

/// A comment extracted from source code, with its anchor
#[derive(Debug, Clone)]
pub struct ExtractedComment {
    pub text: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub is_doc: bool,
    pub is_inline: bool,
    pub anchor: Option<CommentAnchor>,
}

pub trait Language {
    fn parse(&mut self, source: &str) -> ParseResult;
    fn is_valid(&self, result: &ParseResult) -> bool;
    fn find_extra_delimiter(&self, result: &ParseResult) -> Option<Span>;
    fn explain_error(&self, result: &ParseResult, line: usize) -> Option<SyntaxErrorDiagnostic>;
    fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError>;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn diagnostic_message(&self, error: &DelimiterError) -> String;
    fn find_symbol_node(&self, result: &ParseResult, name: &str) -> Option<(usize, usize)>;
    fn find_all_entities(&self, _result: &ParseResult) -> Vec<Entity> { vec![] }
    /// Return byte range of the body (between { }) of a named entity.
    fn entity_body_range(&self, result: &ParseResult, name: &str) -> Option<(usize, usize)> {
        let (start, end) = self.find_symbol_node(result, name)?;
        // Default: return body between first '{' and matching '}'
        let source = &result.text()[start..end];
        let open = source.find('{')?;
        let mut stack = 1;
        let mut close = open + 1;
        while close < source.len() && stack > 0 {
            match source.as_bytes()[close] {
                b'{' => stack += 1,
                b'}' => stack -= 1,
                _ => {}
            }
            close += 1;
        }
        if stack == 0 {
            Some((start + open + 1, start + close))
        } else {
            None
        }
    }

    /// Extract all comment nodes with their anchor entities.
    fn extract_comments(&self, result: &ParseResult) -> Vec<ExtractedComment> {
        let root = result.tree.root_node();
        let mut raw = Vec::new();
        collect_comments(root, &result.source, &mut raw);
        raw.into_iter().filter_map(|c| {
            let node = find_node_at_byte_range(root, c.start_byte, c.end_byte);
            let anchor = node
                .and_then(|n| self.find_enclosing_entity(n, result))
                .map(|(entity_start, _, kind, name)| CommentAnchor {
                    entity_kind: kind,
                    entity_name: name,
                    relative_offset: c.start_byte as isize - entity_start as isize,
                });
            Some(ExtractedComment { anchor, ..c })
        }).collect()
    }

    /// Resolve comment anchors after extraction (default: already done above).
    fn resolve_comment_anchors(&self, _result: &ParseResult, _comments: &mut [ExtractedComment]) {}

    /// Walk up to find the nearest named entity node.
    fn find_enclosing_entity(&self, node: Node<'_>, result: &ParseResult) -> Option<(usize, usize, String, Option<String>)> {
        let kinds = [
            "function_item", "function_declaration", "function_definition",
            "method_definition", "method_declaration",
            "struct_item", "enum_item", "trait_item", "impl_item",
            "class_declaration", "class_definition",
            "interface_declaration", "module", "namespace_declaration",
        ];
        let mut cur = node;
        loop {
            let k = cur.kind();
            if kinds.contains(&k) {
                let name = cur.child_by_field_name("name")
                    .and_then(|n| n.utf8_text(result.text().as_bytes()).ok())
                    .map(|s| s.to_string());
                return Some((cur.start_byte(), cur.end_byte(), k.to_string(), name));
            }
            match cur.parent() {
                Some(p) => cur = p,
                None => return None,
            }
        }
    }
}

fn find_node_at_byte_range<'a>(node: Node<'a>, start: usize, end: usize) -> Option<Node<'a>> {
    if node.start_byte() <= start && node.end_byte() >= end {
        let mut cursor = node.walk();
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i as u32) {
                if let Some(f) = find_node_at_byte_range(child, start, end) { return Some(f); }
            }
        }
        Some(node)
    } else { None }
}

fn collect_comments(node: tree_sitter::Node, source: &str, out: &mut Vec<ExtractedComment>) {
    let k = node.kind();
    if k.contains("comment") && !k.contains("string") {
        if let Ok(t) = node.utf8_text(source.as_bytes()) {
            let is_doc = t.trim_start().starts_with("///") || t.trim_start().starts_with("//!") || t.trim_start().starts_with("/**");
            let row = node.start_position().row;
            let is_inline = source.lines().nth(row).map(|l| {
                l[..node.start_position().column].chars().any(|c| !c.is_whitespace())
            }).unwrap_or(false);
            out.push(ExtractedComment {
                text: t.to_string(),
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
                is_doc,
                is_inline,
                anchor: None,
            });
        }
    }
    for i in 0..node.child_count() {
        if let Some(c) = node.child(i as u32) { collect_comments(c, source, out); }
    }
}

fn collect_comments_filtered(node: tree_sitter::Node, result: &ParseResult, out: &mut Vec<ExtractedComment>) {
    let k = node.kind();
    let is_comment = k == "line_comment" || k == "block_comment";
    let is_inline = is_comment && result.text().lines().nth(node.start_position().row).map(|l| {
        l[..node.start_position().column].chars().any(|c| !c.is_whitespace())
    }).unwrap_or(false);
    if is_comment {
        if let Ok(t) = node.utf8_text(result.text().as_bytes()) {
            let is_doc = t.trim_start().starts_with("///") || t.trim_start().starts_with("//!") || t.trim_start().starts_with("/**");
            out.push(ExtractedComment {
                text: t.to_string(),
                start_byte: node.start_byte(),
                end_byte: node.end_byte(),
                is_doc,
                is_inline,
                anchor: None,
            });
        }
    }
    for i in 0..node.child_count() {
        if let Some(c) = node.child(i as u32) { collect_comments_filtered(c, result, out); }
    }
}

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

fn traverse_for_delimiters(node: Node, stack: &mut Vec<(char, usize, Option<String>)>, errors: &mut Vec<DelimiterError>, index: &LineIndex) {
    let kind = node.kind();
    if kind.contains("string") || kind.contains("comment") || kind == "string_literal" || kind == "raw_string_literal" { return; }
    match kind {
        "(" | "[" | "{" => {
            let close = match kind { "(" => ')', "[" => ']', "{" => '}', _ => unreachable!() };
            let parent_kind = node.parent().map(|p| p.kind().to_string());
            stack.push((close, node.start_byte(), parent_kind));
        }
        ")" | "]" | "}" => {
            let close_char = kind.chars().next().unwrap();
            if let Some((expected, open_byte, _)) = stack.pop() {
                if expected != close_char {
                    errors.push(DelimiterError::Extra { span: Span::from_node(node, index), delimiter: close_char });
                    stack.push((expected, open_byte, None));
                }
            } else { errors.push(DelimiterError::Extra { span: Span::from_node(node, index), delimiter: close_char }); }
        }
        _ => {}
    }
    for child in node.children(&mut node.walk()) { traverse_for_delimiters(child, stack, errors, index); }
}

fn has_error_node(node: Node) -> bool {
    if node.is_error() { return true; }
    for child in node.children(&mut node.walk()) { if has_error_node(child) { return true; } }
    false
}

macro_rules! impl_language {
    ($name:ident, $lang:expr) => {
        pub struct $name { parser: Parser }
        impl $name {
            pub fn new() -> Self { let mut parser = Parser::new(); parser.set_language(&$lang.into()).unwrap(); Self { parser } }
            pub fn parser_mut(&mut self) -> &mut Parser { &mut self.parser }
        }
        impl Language for $name {
            fn parse(&mut self, source: &str) -> ParseResult { let tree = self.parser.parse(source, None).unwrap(); let index = LineIndex::new(source); ParseResult { tree, source: source.to_string(), index } }
            fn is_valid(&self, result: &ParseResult) -> bool { !has_error_node(result.tree.root_node()) }
            fn find_extra_delimiter(&self, _: &ParseResult) -> Option<Span> { None }
            fn explain_error(&self, result: &ParseResult, line: usize) -> Option<SyntaxErrorDiagnostic> { let node = result.node_at_line(line)?; if node.is_error() || node.has_error() { let text = node.utf8_text(result.text().as_bytes()).unwrap_or(""); let details = format!("Syntax error near '{}'", text); let span = Span::from_node(node, &result.index); return Some(SyntaxErrorDiagnostic { src: NamedSource::new("input", result.text().to_string()), error_span: (span.start_byte, span.end_byte - span.start_byte).into(), details }); } None }
            fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError> { find_delimiter_errors_via_ast(result.tree.root_node(), &result.index) }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
            fn diagnostic_message(&self, error: &DelimiterError) -> String { match error { DelimiterError::Extra { delimiter, .. } => format!("Extra '{}'", delimiter), DelimiterError::Missing { expected, .. } => format!("Missing '{}'", expected) } }
            fn find_symbol_node(&self, _result: &ParseResult, _name: &str) -> Option<(usize, usize)> { None }
        }
    };
}

impl_language!(TypeScriptLanguage, tree_sitter_typescript::LANGUAGE_TYPESCRIPT);
impl_language!(JavaScriptLanguage, tree_sitter_javascript::LANGUAGE);

pub struct RustLanguage { parser: Parser }
impl RustLanguage {
    pub fn new() -> Self { let mut parser = Parser::new(); parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap(); Self { parser } }
    pub fn parser_mut(&mut self) -> &mut Parser { &mut self.parser }

    pub fn find_function_body_range(&self, source: &str, function_name: &str) -> Option<(usize, usize)> {
        let pattern = format!("fn {}(", function_name);
        let mut start = 0;
        while let Some(pos) = source[start..].find(&pattern) {
            let abs_pos = start + pos;
            let after_sig = &source[abs_pos..];
            let open_brace_offset = after_sig.find('{')?;
            let open_byte = abs_pos + open_brace_offset;
            let mut stack = 1;
            let mut close_byte = open_byte + 1;
            for c in after_sig[open_brace_offset + 1..].chars() {
                if c == '{' { stack += 1; }
                else if c == '}' { stack -= 1; if stack == 0 { break; } }
                close_byte += c.len_utf8();
            }
            if stack == 0 { return Some((open_byte, close_byte)); }
            start = abs_pos + 1;
        }
        None
    }
}
impl Language for RustLanguage {
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
            return Some(SyntaxErrorDiagnostic {
                src: NamedSource::new("input", result.text().to_string()),
                error_span: (span.start_byte, span.end_byte - span.start_byte).into(),
                details
            });
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

    fn find_symbol_node(&self, result: &ParseResult, name: &str) -> Option<(usize, usize)> {
        let query = tree_sitter::Query::new(
            &tree_sitter_rust::LANGUAGE.into(),
            "(function_item name: (identifier) @name) @item"
        ).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let root = result.tree.root_node();
        let mut matches = cursor.matches(&query, root, result.text().as_bytes());
        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                if capture.node.kind() == "identifier" {
                    if let Ok(text) = capture.node.utf8_text(result.text().as_bytes()) {
                        if text == name {
                            let item = match_.captures.iter()
                                .find(|c| c.node.kind() == "function_item")
                                .unwrap().node;
                            return Some((item.start_byte(), item.end_byte()));
                        }
                    }
                }
            }
        }
        None
    }

    fn find_all_entities(&self, result: &ParseResult) -> Vec<Entity> {
        let query = tree_sitter::Query::new(
            &tree_sitter_rust::LANGUAGE.into(),
            "(function_item name: (identifier) @name) @item"
        ).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let root = result.tree.root_node();
        let mut matches = cursor.matches(&query, root, result.text().as_bytes());
        let mut entities = Vec::new();
        while let Some(m) = matches.next() {
            if let Some(item) = m.captures.iter().find(|c| c.node.kind() == "function_item") {
                if let Some(name_node) = m.captures.iter().find(|c| c.node.kind() == "identifier") {
                    if let Ok(name_text) = name_node.node.utf8_text(result.text().as_bytes()) {
                        entities.push(Entity {
                            name: name_text.to_string(),
                            kind: "function_item".to_string(),
                            start_byte: item.node.start_byte(),
                            end_byte: item.node.end_byte(),
                            signature: item.node.utf8_text(result.text().as_bytes()).unwrap_or("").to_string(),
                        });
                    }
                }
            }
        }
        entities
    }

    fn extract_comments(&self, result: &ParseResult) -> Vec<ExtractedComment> {
        let root = result.tree.root_node();
        let mut raw = Vec::new();
        collect_comments_filtered(root, result, &mut raw);
        raw.into_iter().filter_map(|c| {
            let node = find_node_at_byte_range(root, c.start_byte, c.end_byte);
            let (es, _, k, n) = self.find_enclosing_entity(node?, result)?;
            Some(ExtractedComment {
                anchor: Some(CommentAnchor {
                    entity_kind: k,
                    entity_name: n,
                    relative_offset: c.start_byte as isize - es as isize,
                }),
                ..c
            })
        }).collect()
    }

    fn resolve_comment_anchors(&self, _result: &ParseResult, _comments: &mut [ExtractedComment]) {}

    fn find_enclosing_entity(&self, node: Node<'_>, result: &ParseResult) -> Option<(usize, usize, String, Option<String>)> {
        let kinds = ["function_item","struct_item","enum_item","trait_item","impl_item","static_item","const_item","type_item"];
        let mut cur = node;
        loop {
            let k = cur.kind();
            if kinds.contains(&k) {
                let name = cur.child_by_field_name("name")
                    .and_then(|n| n.utf8_text(result.text().as_bytes()).ok())
                    .map(|s| s.to_string());
                return Some((cur.start_byte(), cur.end_byte(), k.to_string(), name));
            }
            match cur.parent() { Some(p) => cur = p, None => return None }
        }
    }
}
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
impl_language!(SwiftLanguage, tree_sitter_swift::LANGUAGE);
impl_language!(ScalaLanguage, tree_sitter_scala::LANGUAGE);
impl_language!(ZigLanguage, tree_sitter_zig::LANGUAGE);

pub fn detect_language(file_path: &std::path::Path) -> anyhow::Result<Box<dyn Language>> {
    match file_path.extension().and_then(|e| e.to_str()) {
        Some("rs") => Ok(Box::new(RustLanguage::new())),
        Some("ts")|Some("tsx")|Some("mts")|Some("cts") => Ok(Box::new(TypeScriptLanguage::new())),
        Some("js")|Some("jsx")|Some("mjs")|Some("cjs") => Ok(Box::new(JavaScriptLanguage::new())),
        Some("py")|Some("pyi") => Ok(Box::new(PythonLanguage::new())),
        Some("go") => Ok(Box::new(GoLanguage::new())),
        Some("rb") => Ok(Box::new(RubyLanguage::new())),
        Some("php") => Ok(Box::new(PHPLanguage::new())),
        Some("html")|Some("htm") => Ok(Box::new(HtmlLanguage::new())),
        Some("xml") => Ok(Box::new(XmlLanguage::new())),
        Some("c")|Some("h") => Ok(Box::new(CLanguage::new())),
        Some("cpp")|Some("cc")|Some("cxx")|Some("hpp") => Ok(Box::new(CppLanguage::new())),
        Some("java") => Ok(Box::new(JavaLanguage::new())),
        Some("cs") => Ok(Box::new(CSharpLanguage::new())),
        Some("swift") => Ok(Box::new(SwiftLanguage::new())),
        Some("scala") => Ok(Box::new(ScalaLanguage::new())),
        Some("zig") => Ok(Box::new(ZigLanguage::new())),
        _ => anyhow::bail!("Unsupported file extension."),
    }
}
