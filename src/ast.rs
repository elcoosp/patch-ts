use line_index::{LineIndex, TextSize};
use miette::NamedSource;
use tree_sitter::{Node, Parser, StreamingIterator, Tree};

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
        let start = index.line_col(TextSize::from(crate::num::to_u32(node.start_byte())));
        let end = index.line_col(TextSize::from(crate::num::to_u32(node.end_byte())));
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
        let start = index.line_col(TextSize::from(crate::num::to_u32(start_byte)));
        let end = index.line_col(TextSize::from(crate::num::to_u32(end_byte)));
        Span {
            start_byte,
            end_byte,
            start_line: start.line as usize + 1,
            start_column: start.col as usize + 1,
            end_line: end.line as usize + 1,
            end_column: end.col as usize + 1,
        }
    }

    #[must_use]
    pub fn shift(&self, offset: isize) -> Self {
        let shift = |v: usize| crate::num::to_usize(crate::num::to_isize(v) + offset);
        Span {
            start_byte: shift(self.start_byte),
            end_byte: shift(self.end_byte),
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
    #[must_use]
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
    #[must_use]
    pub fn delta(&self) -> isize {
        match self {
            DelimiterError::Extra { span, .. } => {
                -crate::num::isize_diff(span.end_byte, span.start_byte)
            }
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
    #[must_use]
    pub fn text(&self) -> &str {
        &self.source
    }
    #[must_use]
    pub fn node_at_line(&self, line: usize) -> Option<Node<'_>> {
        let root = self.tree.root_node();
        find_node_at_line(root, line, &self.index)
    }
}

fn find_node_at_line<'a>(
    node: Node<'a>,
    target_line: usize,
    index: &LineIndex,
) -> Option<Node<'a>> {
    let start_byte = node.start_byte();
    let start_pos = index.line_col(TextSize::from(crate::num::to_u32(start_byte)));
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
    fn find_all_entities(&self, _result: &ParseResult) -> Vec<Entity> {
        vec![]
    }
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
            // `close` now sits just past the matching `}`; the body span must
            // stop *before* the closing brace so it is preserved on replace.
            Some((start + open + 1, start + close - 1))
        } else {
            None
        }
    }

    /// Extract all comment nodes with their anchor entities.
    fn extract_comments(&self, result: &ParseResult) -> Vec<ExtractedComment> {
        let root = result.tree.root_node();
        let mut raw = Vec::new();
        collect_comments(root, &result.source, &mut raw);
        raw.into_iter()
            .map(|c| {
                let node = find_node_at_byte_range(root, c.start_byte, c.end_byte);
                let anchor = node
                    .and_then(|n| self.find_enclosing_entity(n, result))
                    .map(|(entity_start, _, kind, name)| CommentAnchor {
                        entity_kind: kind,
                        entity_name: name,
                        relative_offset: crate::num::isize_diff(c.start_byte, entity_start),
                    });
                ExtractedComment { anchor, ..c }
            })
            .collect()
    }

    /// Resolve comment anchors after extraction (default: already done above).
    fn resolve_comment_anchors(&self, _result: &ParseResult, _comments: &mut [ExtractedComment]) {}

    /// Walk up to find the nearest named entity node.
    fn find_enclosing_entity(
        &self,
        node: Node<'_>,
        result: &ParseResult,
    ) -> Option<(usize, usize, String, Option<String>)> {
        let kinds = [
            "function_item",
            "function_declaration",
            "function_definition",
            "method_definition",
            "method_declaration",
            "struct_item",
            "enum_item",
            "trait_item",
            "impl_item",
            "class_declaration",
            "class_definition",
            "interface_declaration",
            "module",
            "namespace_declaration",
        ];
        let mut cur = node;
        loop {
            let k = cur.kind();
            if kinds.contains(&k) {
                let name = cur
                    .child_by_field_name("name")
                    .and_then(|n| n.utf8_text(result.text().as_bytes()).ok())
                    .map(std::string::ToString::to_string);
                return Some((cur.start_byte(), cur.end_byte(), k.to_string(), name));
            }
            {
                let p = cur.parent()?;
                cur = p;
            }
        }
    }
}

fn find_node_at_byte_range(node: Node<'_>, start: usize, end: usize) -> Option<Node<'_>> {
    if node.start_byte() <= start && node.end_byte() >= end {
        let _cursor = node.walk();
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i)
                && let Some(f) = find_node_at_byte_range(child, start, end)
            {
                return Some(f);
            }
        }
        Some(node)
    } else {
        None
    }
}

fn collect_function_ranges(node: Node, source: &[u8], name: &str, out: &mut Vec<(usize, usize)>) {
    if node.kind() == "function_item"
        && let Some(name_node) = node.child_by_field_name("name")
        && name_node.utf8_text(source).is_ok_and(|t| t == name)
    {
        out.push((node.start_byte(), node.end_byte()));
    }
    for i in 0..node.child_count() {
        if let Some(c) = node.child(i) {
            collect_function_ranges(c, source, name, out);
        }
    }
}

fn collect_comments(node: tree_sitter::Node, source: &str, out: &mut Vec<ExtractedComment>) {
    let k = node.kind();
    if k.contains("comment")
        && !k.contains("string")
        && let Ok(t) = node.utf8_text(source.as_bytes())
    {
        let is_doc = t.trim_start().starts_with("///")
            || t.trim_start().starts_with("//!")
            || t.trim_start().starts_with("/**");
        let row = node.start_position().row;
        let is_inline = source.lines().nth(row).is_some_and(|l| {
            l[..node.start_position().column]
                .chars()
                .any(|c| !c.is_whitespace())
        });
        out.push(ExtractedComment {
            text: t.to_string(),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            is_doc,
            is_inline,
            anchor: None,
        });
    }
    for i in 0..node.child_count() {
        if let Some(c) = node.child(i) {
            collect_comments(c, source, out);
        }
    }
}

fn collect_comments_filtered(
    node: tree_sitter::Node,
    result: &ParseResult,
    out: &mut Vec<ExtractedComment>,
) {
    let k = node.kind();
    let is_comment = k == "line_comment" || k == "block_comment";
    let is_inline = is_comment
        && result
            .text()
            .lines()
            .nth(node.start_position().row)
            .is_some_and(|l| {
                l[..node.start_position().column]
                    .chars()
                    .any(|c| !c.is_whitespace())
            });
    if is_comment && let Ok(t) = node.utf8_text(result.text().as_bytes()) {
        let is_doc = t.trim_start().starts_with("///")
            || t.trim_start().starts_with("//!")
            || t.trim_start().starts_with("/**");
        out.push(ExtractedComment {
            text: t.to_string(),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
            is_doc,
            is_inline,
            anchor: None,
        });
    }
    for i in 0..node.child_count() {
        if let Some(c) = node.child(i) {
            collect_comments_filtered(c, result, out);
        }
    }
}

pub(crate) fn find_delimiter_errors_via_ast(
    root: Node,
    index: &LineIndex,
    source: &str,
) -> Vec<DelimiterError> {
    let mut errors = Vec::new();
    let mut stack: Vec<(char, usize, Option<String>)> = Vec::new();
    traverse_for_delimiters(root, &mut stack, &mut errors, index);
    collect_missing_delimiters(root, source, &mut errors, index);
    // Leftover openers without a matching tree-sitter MISSING node: insert the
    // closer at the end of the source (just before any trailing newline), which
    // is where the parser expected it.
    let mut end = root.end_byte();
    while end > 0 && source.as_bytes()[end - 1] == b'\n' && source[end..].trim().is_empty() {
        end -= 1;
    }
    for (expected, _open_byte, parent_kind) in stack {
        errors.push(DelimiterError::Missing {
            expected,
            insert_at: Span::from_byte_range(end, end, index),
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
    if kind.contains("string")
        || kind.contains("comment")
        || kind == "string_literal"
        || kind == "raw_string_literal"
    {
        return;
    }
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
    for child in node.children(&mut node.walk()) {
        traverse_for_delimiters(child, stack, errors, index);
    }
}

fn has_error_node(node: Node) -> bool {
    if node.is_error() || node.is_missing() {
        return true;
    }
    for child in node.children(&mut node.walk()) {
        if has_error_node(child) {
            return true;
        }
    }
    false
}

/// Harvest tree-sitter `MISSING` delimiter nodes as repairable errors.
///
/// When tree-sitter recovers from an unclosed delimiter it inserts a virtual
/// `MISSING` token at the exact byte position where the closer belongs. These
/// are invisible to `is_error()` but are precisely the repairs `balance`
/// exists to make.
fn collect_missing_delimiters(
    node: Node,
    source: &str,
    errors: &mut Vec<DelimiterError>,
    index: &LineIndex,
) {
    if node.is_missing() {
        let kind = node.kind();
        if let Some(ch) = kind.chars().next()
            && matches!(ch, '(' | '[' | '{' | ')' | ']' | '}')
            && kind.chars().count() == 1
        {
            let mut pos = node.start_byte();
            // Avoid appending the closer after a trailing newline at EOF:
            // `x = (1 + 2\n` should get `)` on the same line, and unclosed
            // blocks get their `}` on a line of its own instead of after `\n`.
            while pos > 0 && source.as_bytes()[pos - 1] == b'\n' && source[pos..].trim().is_empty()
            {
                pos -= 1;
            }
            errors.push(DelimiterError::Missing {
                expected: ch,
                insert_at: Span::from_byte_range(pos, pos, index),
                parent_kind: node.parent().map(|p| p.kind().to_string()),
            });
        }
    }
    for i in 0..node.child_count() {
        if let Some(c) = node.child(i) {
            collect_missing_delimiters(c, source, errors, index);
        }
    }
}

macro_rules! impl_language {
    ($name:ident, $lang:expr) => {
        pub struct $name {
            parser: Parser,
        }
        impl $name {
            pub fn new() -> Self {
                let mut parser = Parser::new();
                parser.set_language(&$lang.into()).unwrap();
                Self { parser }
            }
            pub fn parser_mut(&mut self) -> &mut Parser {
                &mut self.parser
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl Language for $name {
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
            fn find_extra_delimiter(&self, _: &ParseResult) -> Option<Span> {
                None
            }
            fn explain_error(
                &self,
                result: &ParseResult,
                line: usize,
            ) -> Option<SyntaxErrorDiagnostic> {
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
                find_delimiter_errors_via_ast(result.tree.root_node(), &result.index, result.text())
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
            fn find_symbol_node(
                &self,
                _result: &ParseResult,
                _name: &str,
            ) -> Option<(usize, usize)> {
                None
            }
        }
    };
}

impl_language!(
    TypeScriptLanguage,
    tree_sitter_typescript::LANGUAGE_TYPESCRIPT
);
impl_language!(JavaScriptLanguage, tree_sitter_javascript::LANGUAGE);

pub struct RustLanguage {
    parser: Parser,
}
impl RustLanguage {
    #[must_use]
    ///
    /// # Panics
    ///
    /// Panics if an internal `unwrap` fails (invariant violation).
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .unwrap();
        Self { parser }
    }
    pub fn parser_mut(&mut self) -> &mut Parser {
        &mut self.parser
    }

    #[must_use]
    /// Locate the byte range of the function named `function_name`.
    ///
    /// Uses the parsed syntax tree instead of brace counting so that
    /// *unbalanced* functions (the exact cases `balance` exists to repair)
    /// are still located, and duplicate definitions are treated as ambiguous
    /// (`None`).
    pub fn find_function_body_range(
        &self,
        source: &str,
        function_name: &str,
    ) -> Option<(usize, usize)> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .ok()?;
        let tree = parser.parse(source, None)?;
        let root = tree.root_node();
        // Exact lookup: duplicate definitions are ambiguous.
        let mut matches: Vec<(usize, usize)> = Vec::new();
        collect_function_ranges(root, source.as_bytes(), function_name, &mut matches);
        if matches.len() == 1 {
            return matches.into_iter().next();
        }
        if matches.len() > 1 {
            return None;
        }
        // Fallback: an unbalanced body (the cases `balance` exists to repair)
        // never forms a `function_item` node. Use the top-level ERROR node
        // containing its declaration instead, if unique — the root itself may
        // be that ERROR node when the whole file fails to structure.
        let pattern = format!("fn {function_name}(");
        let contains_decl = |n: Node<'_>| {
            n.utf8_text(source.as_bytes())
                .is_ok_and(|t| t.contains(&pattern))
        };
        let error_ranges: Vec<(usize, usize)> = (0..root.child_count())
            .filter_map(|i| root.child(i))
            .chain(std::iter::once(root))
            .filter(tree_sitter::Node::is_error)
            .filter(|c| contains_decl(*c))
            .map(|c| (c.start_byte(), c.end_byte()))
            .collect();
        if error_ranges.len() == 1 {
            error_ranges.into_iter().next()
        } else {
            None
        }
    }
}

impl Default for RustLanguage {
    fn default() -> Self {
        Self::new()
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
    fn find_extra_delimiter(&self, _: &ParseResult) -> Option<Span> {
        None
    }
    fn explain_error(&self, result: &ParseResult, line: usize) -> Option<SyntaxErrorDiagnostic> {
        let node = result.node_at_line(line)?;
        if node.is_error() || node.has_error() {
            let text = node.utf8_text(result.text().as_bytes()).unwrap_or("");
            let details = format!("Syntax error near '{text}'");
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
        find_delimiter_errors_via_ast(result.tree.root_node(), &result.index, result.text())
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn diagnostic_message(&self, error: &DelimiterError) -> String {
        match error {
            DelimiterError::Extra { delimiter, .. } => format!("Extra '{delimiter}'"),
            DelimiterError::Missing { expected, .. } => format!("Missing '{expected}'"),
        }
    }

    fn find_symbol_node(&self, result: &ParseResult, name: &str) -> Option<(usize, usize)> {
        let query = tree_sitter::Query::new(
            &tree_sitter_rust::LANGUAGE.into(),
            "(function_item name: (identifier) @name) @item",
        )
        .unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let root = result.tree.root_node();
        let mut matches = cursor.matches(&query, root, result.text().as_bytes());
        while let Some(match_) = matches.next() {
            for capture in match_.captures() {
                if capture.node.kind() == "identifier"
                    && let Ok(text) = capture.node.utf8_text(result.text().as_bytes())
                    && text == name
                {
                    let item = match_
                        .captures()
                        .iter()
                        .find(|c| c.node.kind() == "function_item")
                        .unwrap()
                        .node;
                    return Some((item.start_byte(), item.end_byte()));
                }
            }
        }
        None
    }

    fn find_all_entities(&self, result: &ParseResult) -> Vec<Entity> {
        let query = tree_sitter::Query::new(
            &tree_sitter_rust::LANGUAGE.into(),
            "(function_item name: (identifier) @name) @item",
        )
        .unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let root = result.tree.root_node();
        let mut matches = cursor.matches(&query, root, result.text().as_bytes());
        let mut entities = Vec::new();
        while let Some(m) = matches.next() {
            if let Some(item) = m
                .captures()
                .iter()
                .find(|c| c.node.kind() == "function_item")
                && let Some(name_node) = m.captures().iter().find(|c| c.node.kind() == "identifier")
                && let Ok(name_text) = name_node.node.utf8_text(result.text().as_bytes())
            {
                entities.push(Entity {
                    name: name_text.to_string(),
                    kind: "function_item".to_string(),
                    start_byte: item.node.start_byte(),
                    end_byte: item.node.end_byte(),
                    signature: item
                        .node
                        .utf8_text(result.text().as_bytes())
                        .unwrap_or("")
                        .to_string(),
                });
            }
        }
        entities
    }

    fn extract_comments(&self, result: &ParseResult) -> Vec<ExtractedComment> {
        let root = result.tree.root_node();
        let mut raw = Vec::new();
        collect_comments_filtered(root, result, &mut raw);
        raw.into_iter()
            .filter_map(|c| {
                let node = find_node_at_byte_range(root, c.start_byte, c.end_byte);
                let (es, _, k, n) = self.find_enclosing_entity(node?, result)?;
                Some(ExtractedComment {
                    anchor: Some(CommentAnchor {
                        entity_kind: k,
                        entity_name: n,
                        relative_offset: crate::num::isize_diff(c.start_byte, es),
                    }),
                    ..c
                })
            })
            .collect()
    }

    fn resolve_comment_anchors(&self, _result: &ParseResult, _comments: &mut [ExtractedComment]) {}

    fn find_enclosing_entity(
        &self,
        node: Node<'_>,
        result: &ParseResult,
    ) -> Option<(usize, usize, String, Option<String>)> {
        let kinds = [
            "function_item",
            "struct_item",
            "enum_item",
            "trait_item",
            "impl_item",
            "static_item",
            "const_item",
            "type_item",
        ];
        let mut cur = node;
        loop {
            let k = cur.kind();
            if kinds.contains(&k) {
                let name = cur
                    .child_by_field_name("name")
                    .and_then(|n| n.utf8_text(result.text().as_bytes()).ok())
                    .map(std::string::ToString::to_string);
                return Some((cur.start_byte(), cur.end_byte(), k.to_string(), name));
            }
            {
                let p = cur.parent()?;
                cur = p;
            }
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

///
/// # Errors
///
/// Returns an error if the content cannot be generated from the input.
pub fn detect_language(file_path: &std::path::Path) -> anyhow::Result<Box<dyn Language>> {
    match file_path.extension().and_then(|e| e.to_str()) {
        Some("rs") => Ok(Box::new(RustLanguage::new())),
        Some("ts" | "tsx" | "mts" | "cts") => Ok(Box::new(TypeScriptLanguage::new())),
        Some("js" | "jsx" | "mjs" | "cjs") => Ok(Box::new(JavaScriptLanguage::new())),
        Some("py" | "pyi") => Ok(Box::new(PythonLanguage::new())),
        Some("go") => Ok(Box::new(GoLanguage::new())),
        Some("rb") => Ok(Box::new(RubyLanguage::new())),
        Some("php") => Ok(Box::new(PHPLanguage::new())),
        Some("html" | "htm") => Ok(Box::new(HtmlLanguage::new())),
        Some("xml") => Ok(Box::new(XmlLanguage::new())),
        Some("c" | "h") => Ok(Box::new(CLanguage::new())),
        Some("cpp" | "cc" | "cxx" | "hpp") => Ok(Box::new(CppLanguage::new())),
        Some("java") => Ok(Box::new(JavaLanguage::new())),
        Some("cs") => Ok(Box::new(CSharpLanguage::new())),
        Some("swift") => Ok(Box::new(SwiftLanguage::new())),
        Some("scala") => Ok(Box::new(ScalaLanguage::new())),
        Some("zig") => Ok(Box::new(ZigLanguage::new())),
        _ => anyhow::bail!("Unsupported file extension."),
    }
}
