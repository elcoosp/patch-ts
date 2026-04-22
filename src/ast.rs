use line_index::{LineIndex, TextSize};
use miette::NamedSource;
use tree_sitter::{Node, Parser, Tree, Query, QueryCursor};
use tree_sitter::StreamingIterator;

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

pub struct ParseResult {
    pub tree: Tree,
    pub source: String,
    pub index: LineIndex,
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
    fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError>;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    /// Return a human-readable diagnostic message for a delimiter error.
    fn diagnostic_message(&self, error: &DelimiterError) -> String;
}

// ----------------------------------------------------------------------
// AST‑based delimiter detection – works for all languages
// ----------------------------------------------------------------------

/// Traverse the AST to find delimiter errors, skipping string literals and comments.
pub(crate) fn find_delimiter_errors_via_ast(root: Node, index: &LineIndex) -> Vec<DelimiterError> {
    let mut errors = Vec::new();
    let mut stack: Vec<(char, usize, Option<String>)> = Vec::new(); // (expected_close, open_byte, parent_kind)
    traverse_for_delimiters(root, &mut stack, &mut errors, index);

    // Any remaining open delimiters are missing
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

    // Skip string literals and comments entirely – they can't contain real delimiters
    if kind.contains("string") || kind.contains("comment") || kind == "string_literal" || kind == "raw_string_literal" {
        return;
    }

    // Process delimiter nodes
    match kind {
        "(" | "[" | "{" => {
            let close = match kind {
                "(" => ')',
                "[" => ']',
                "{" => '}',
                _ => unreachable!(),
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
                    // Put the expected back
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

    // Recurse into children
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        traverse_for_delimiters(child, stack, errors, index);
    }
}
// ----------------------------------------------------------------------
// Free functions
// ----------------------------------------------------------------------

fn has_error_node(node: Node) -> bool {
    if node.is_error() {
        return true;
    }
    for child in node.children(&mut node.walk()) {
        if has_error_node(child) {
            return true;
        }
    }
    false
}

pub(crate) fn scan_delimiter_errors_full(source: &str, index: &LineIndex) -> Vec<DelimiterError> {
    let mut errors = Vec::new();
    let mut stack: Vec<(char, usize)> = Vec::new();

    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    let mut in_char = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut escape = false;

    while i < chars.len() {
        let c = chars[i];

        if !in_string && !in_char && !in_line_comment && !in_block_comment {
            if c == '/' && i + 1 < chars.len() {
                if chars[i + 1] == '/' {
                    in_line_comment = true;
                    i += 2;
                    continue;
                } else if chars[i + 1] == '*' {
                    in_block_comment = true;
                    i += 2;
                    continue;
                }
            }
        }

        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
            }
            i += 1;
            continue;
        }

        if in_block_comment {
            if c == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                in_block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }

        if in_string {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if in_char {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '\'' {
                in_char = false;
            }
            i += 1;
            continue;
        }

        if c == '"' {
            in_string = true;
            i += 1;
            continue;
        }
        if c == '\'' {
            in_char = true;
            i += 1;
            continue;
        }

        let byte_pos = source[..i].len();
        match c {
            '(' | '[' | '{' => {
                let close = match c {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    _ => unreachable!(),
                };
                stack.push((close, byte_pos));
            }
            ')' | ']' | '}' => {
                if let Some((expected, open_byte)) = stack.pop() {
                    if expected != c {
                        errors.push(DelimiterError::Extra {
                            span: Span::from_byte_range(byte_pos, byte_pos + 1, index),
                            delimiter: c,
                        });
                        stack.push((expected, open_byte));
                    }
                } else {
                    errors.push(DelimiterError::Extra {
                        span: Span::from_byte_range(byte_pos, byte_pos + 1, index),
                        delimiter: c,
                    });
                }
            }
            _ => {}
        }
        i += 1;
    }

    for (expected, open_byte) in stack {
        errors.push(DelimiterError::Missing {
            expected,
            insert_at: Span::from_byte_range(open_byte, open_byte + 1, index),
            parent_kind: None,
        });
    }

    errors
}

pub(crate) fn scan_extra_delimiter_errors(source: &str, index: &LineIndex) -> Vec<DelimiterError> {
    let mut errors = Vec::new();
    let mut stack: Vec<(char, usize)> = Vec::new();

    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    let mut in_string = false;
    let mut in_char = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;
    let mut escape = false;

    while i < chars.len() {
        let c = chars[i];

        if !in_string && !in_char && !in_line_comment && !in_block_comment {
            if c == '/' && i + 1 < chars.len() {
                if chars[i + 1] == '/' {
                    in_line_comment = true;
                    i += 2;
                    continue;
                } else if chars[i + 1] == '*' {
                    in_block_comment = true;
                    i += 2;
                    continue;
                }
            }
        }

        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
            }
            i += 1;
            continue;
        }

        if in_block_comment {
            if c == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                in_block_comment = false;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }

        if in_string {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if in_char {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '\'' {
                in_char = false;
            }
            i += 1;
            continue;
        }

        if c == '"' {
            in_string = true;
            i += 1;
            continue;
        }
        if c == '\'' {
            in_char = true;
            i += 1;
            continue;
        }

        let byte_pos = source[..i].len();
        match c {
            '(' | '[' | '{' => {
                let close = match c {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    _ => unreachable!(),
                };
                stack.push((close, byte_pos));
            }
            ')' | ']' | '}' => {
                if let Some((expected, open_byte)) = stack.pop() {
                    if expected != c {
                        errors.push(DelimiterError::Extra {
                            span: Span::from_byte_range(byte_pos, byte_pos + 1, index),
                            delimiter: c,
                        });
                        stack.push((expected, open_byte));
                    }
                } else {
                    errors.push(DelimiterError::Extra {
                        span: Span::from_byte_range(byte_pos, byte_pos + 1, index),
                        delimiter: c,
                    });
                }
            }
            _ => {}
        }
        i += 1;
    }

    errors
}

fn find_missing_delimiters(root: Node, source: &str, index: &LineIndex, lang: tree_sitter::Language) -> Vec<DelimiterError> {
    let mut errors = Vec::new();

    let query_str = r#"
    (MISSING ")") @missing_paren
    (MISSING "}") @missing_brace
    (MISSING "]") @missing_bracket
    "#;

    let query = match Query::new(&lang, query_str) {
        Ok(q) => q,
        Err(_) => return errors,
    };

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, root, source.as_bytes());

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let node = capture.node;
            let parent = node.parent();
            let parent_kind = parent.map(|p| p.kind().to_string());
            let delimiter = match capture.index {
                0 => ')',
                1 => '}',
                2 => ']',
                _ => continue,
            };
            errors.push(DelimiterError::Missing {
                expected: delimiter,
                insert_at: Span::from_node(node, index),
                parent_kind,
            });
        }
    }

    errors
}

// ----------------------------------------------------------------------
// RustLanguage
// ----------------------------------------------------------------------

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
            let mut in_string = false;
            let mut in_char = false;
            let mut escape = false;

            for c in chars {
                let char_len = c.len_utf8();
                if !in_string && !in_char {
                    if c == '"' {
                        in_string = true;
                    } else if c == '\'' {
                        in_char = true;
                    } else if c == '{' {
                        stack += 1;
                    } else if c == '}' {
                        stack -= 1;
                        if stack == 0 {
                            break;
                        }
                    }
                } else {
                    if escape {
                        escape = false;
                    } else if c == '\\' {
                        escape = true;
                    } else if (in_string && c == '"') || (in_char && c == '\'') {
                        in_string = false;
                        in_char = false;
                    }
                }
                close_byte += char_len;
            }

            let range = (open_byte, close_byte);
            if found_range.is_some() {
                return None;
            }
            found_range = Some(range);
            start = abs_pos + 1;
        }

        found_range
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
        !has_error_node(result.tree.root_node())
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

    fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError> {
        find_delimiter_errors_via_ast(result.tree.root_node(), &result.index)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn diagnostic_message(&self, error: &DelimiterError) -> String {
        match error {
            DelimiterError::Extra { delimiter, .. } => format!("Extra '{}'", delimiter),
            DelimiterError::Missing { expected, .. } => format!("Missing '{}'", expected),
        }
    }
}

// ----------------------------------------------------------------------
// TypeScriptLanguage
// ----------------------------------------------------------------------

pub struct TypeScriptLanguage {
    parser: Parser,
}

impl TypeScriptLanguage {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .expect("failed to load TypeScript grammar");
        Self { parser }
    }
}

impl Language for TypeScriptLanguage {
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
        !has_error_node(result.tree.root_node())
    }

    fn find_extra_delimiter(&self, _result: &ParseResult) -> Option<Span> {
        None
    }

    fn explain_error(&self, result: &ParseResult, line: usize) -> Option<SyntaxErrorDiagnostic> {
        let node = result.node_at_line(line)?;
        if node.is_error() || node.has_error() {
            let text = node.utf8_text(result.text().as_bytes()).unwrap_or("");
            let details = format!("Syntax error near '{}'", text);
            let span = Span::from_node(node, &result.index);
            return Some(SyntaxErrorDiagnostic {
                src: NamedSource::new("input", result.text().to_string()),
                error_span: (span.start_byte, span.end_byte - span.start_byte).into(),
                details,
            });
        }
        None
    }

    fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError> {
        let mut errors = scan_extra_delimiter_errors(result.text(), &result.index);
        let root = result.tree.root_node();
        let missing = find_missing_delimiters(root, result.text(), &result.index, tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());
        errors.extend(missing);
        errors
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn diagnostic_message(&self, error: &DelimiterError) -> String {
        match error {
            DelimiterError::Missing { expected, parent_kind, .. } => {
                if let Some(kind) = parent_kind {
                    match kind.as_str() {
                        "function_body" => format!("Missing '{}' for function body", expected),
                        "block" => format!("Missing '{}' for block", expected),
                        "arguments" => format!("Missing '{}' for function arguments", expected),
                        "parenthesized_expression" => format!("Missing '{}' for expression", expected),
                        _ => format!("Missing '{}'", expected),
                    }
                } else {
                    format!("Missing '{}'", expected)
                }
            }
            DelimiterError::Extra { delimiter, .. } => format!("Extra '{}'", delimiter),
        }
    }
}

// ----------------------------------------------------------------------
// JavaScriptLanguage
// ----------------------------------------------------------------------

pub struct JavaScriptLanguage {
    parser: Parser,
}

impl JavaScriptLanguage {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_javascript::LANGUAGE.into())
            .expect("failed to load JavaScript grammar");
        Self { parser }
    }
}

impl Language for JavaScriptLanguage {
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
        !has_error_node(result.tree.root_node())
    }

    fn find_extra_delimiter(&self, _result: &ParseResult) -> Option<Span> {
        None
    }

    fn explain_error(&self, result: &ParseResult, line: usize) -> Option<SyntaxErrorDiagnostic> {
        let node = result.node_at_line(line)?;
        if node.is_error() || node.has_error() {
            let text = node.utf8_text(result.text().as_bytes()).unwrap_or("");
            let details = format!("Syntax error near '{}'", text);
            let span = Span::from_node(node, &result.index);
            return Some(SyntaxErrorDiagnostic {
                src: NamedSource::new("input", result.text().to_string()),
                error_span: (span.start_byte, span.end_byte - span.start_byte).into(),
                details,
            });
        }
        None
    }

    fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError> {
        let mut errors = scan_extra_delimiter_errors(result.text(), &result.index);
        let root = result.tree.root_node();
        let missing = find_missing_delimiters(root, result.text(), &result.index, tree_sitter_javascript::LANGUAGE.into());
        errors.extend(missing);
        errors
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn diagnostic_message(&self, error: &DelimiterError) -> String {
        match error {
            DelimiterError::Missing { expected, parent_kind, .. } => {
                if let Some(kind) = parent_kind {
                    match kind.as_str() {
                        "function_body" => format!("Missing '{}' for function body", expected),
                        "block" => format!("Missing '{}' for block", expected),
                        "arguments" => format!("Missing '{}' for function arguments", expected),
                        "parenthesized_expression" => format!("Missing '{}' for expression", expected),
                        _ => format!("Missing '{}'", expected),
                    }
                } else {
                    format!("Missing '{}'", expected)
                }
            }
            DelimiterError::Extra { delimiter, .. } => format!("Extra '{}'", delimiter),
        }
    }
}

// ----------------------------------------------------------------------
// PythonLanguage
// ----------------------------------------------------------------------
pub struct PythonLanguage { parser: Parser }
impl PythonLanguage {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_python::LANGUAGE.into()).unwrap();
        Self { parser }
    }
}
impl Language for PythonLanguage {
    fn parse(&mut self, source: &str) -> ParseResult {
        let tree = self.parser.parse(source, None).unwrap();
        let index = LineIndex::new(source);
        ParseResult { tree, source: source.to_string(), index }
    }
    fn is_valid(&self, result: &ParseResult) -> bool { !has_error_node(result.tree.root_node()) }
    fn find_extra_delimiter(&self, _result: &ParseResult) -> Option<Span> { None }
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
            DelimiterError::Missing { expected, parent_kind, .. } => {
                if let Some(kind) = parent_kind {
                    match kind.as_str() {
                        "block" => format!("Missing '{}' for block", expected),
                        "parameters" => format!("Missing '{}' for parameters", expected),
                        "parenthesized_expression" => format!("Missing '{}' for expression", expected),
                        _ => format!("Missing '{}'", expected),
                    }
                } else { format!("Missing '{}'", expected) }
            }
            DelimiterError::Extra { delimiter, .. } => format!("Extra '{}'", delimiter),
        }
    }
}

// ----------------------------------------------------------------------
// GoLanguage
// ----------------------------------------------------------------------
pub struct GoLanguage { parser: Parser }
impl GoLanguage {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_go::LANGUAGE.into()).unwrap();
        Self { parser }
    }
}
impl Language for GoLanguage {
    fn parse(&mut self, source: &str) -> ParseResult {
        let tree = self.parser.parse(source, None).unwrap();
        let index = LineIndex::new(source);
        ParseResult { tree, source: source.to_string(), index }
    }
    fn is_valid(&self, result: &ParseResult) -> bool { !has_error_node(result.tree.root_node()) }
    fn find_extra_delimiter(&self, _result: &ParseResult) -> Option<Span> { None }
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
            DelimiterError::Missing { expected, parent_kind, .. } => {
                if let Some(kind) = parent_kind {
                    match kind.as_str() {
                        "block" => format!("Missing '{}' for block", expected),
                        "parameter_list" => format!("Missing '{}' for parameter list", expected),
                        "parenthesized_expression" => format!("Missing '{}' for expression", expected),
                        _ => format!("Missing '{}'", expected),
                    }
                } else { format!("Missing '{}'", expected) }
            }
            DelimiterError::Extra { delimiter, .. } => format!("Extra '{}'", delimiter),
        }
    }
}
