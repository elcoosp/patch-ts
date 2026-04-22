use line_index::{LineIndex, TextSize};
use miette::NamedSource;
use tree_sitter::{Node, Parser, Tree};
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
    },
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
    fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError>;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
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

    fn scan_delimiter_errors(&self, source: &str, index: &LineIndex) -> Vec<DelimiterError> {
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
            });
        }

        errors
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
                return None; // ambiguous
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

    fn find_delimiter_errors(&self, result: &ParseResult) -> Vec<DelimiterError> {
        self.scan_delimiter_errors(result.text(), &result.index)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
