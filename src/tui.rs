use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::panic;
use tree_sitter::{Node, Parser};

/// Walk the entire AST, collecting highlighted spans for leaf tokens.
pub fn highlight(code: &str, lang: &str) -> Result<Vec<(String, Style)>> {
    let mut parser = Parser::new();
    let (language, _is_tsx) = match lang {
        "rust" => (tree_sitter_rust::LANGUAGE.into(), false),
        "typescript" | "tsx" => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            lang == "tsx",
        ),
        "javascript" | "jsx" => (tree_sitter_javascript::LANGUAGE.into(), lang == "jsx"),
        "python" => (tree_sitter_python::LANGUAGE.into(), false),
        "go" => (tree_sitter_go::LANGUAGE.into(), false),
        "ruby" => (tree_sitter_ruby::LANGUAGE.into(), false),
        "php" => (tree_sitter_php::LANGUAGE_PHP.into(), false),
        "html" => (tree_sitter_html::LANGUAGE.into(), false),
        "xml" => (tree_sitter_xml::LANGUAGE_XML.into(), false),
        "c" => (tree_sitter_c::LANGUAGE.into(), false),
        "cpp" => (tree_sitter_cpp::LANGUAGE.into(), false),
        "java" => (tree_sitter_java::LANGUAGE.into(), false),
        "csharp" => (tree_sitter_c_sharp::LANGUAGE.into(), false),
        "swift" => (tree_sitter_swift::LANGUAGE.into(), false),
        "scala" => (tree_sitter_scala::LANGUAGE.into(), false),
        "zig" => (tree_sitter_zig::LANGUAGE.into(), false),
        _ => anyhow::bail!("Unsupported language: {}", lang),
    };
    parser.set_language(&language)?;
    let tree = parser
        .parse(code, None)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse code"))?;
    let root = tree.root_node();
    let mut spans = Vec::new();
    collect_styled_spans(&root, code, &mut spans);
    Ok(spans)
}

fn collect_styled_spans(node: &Node, source: &str, spans: &mut Vec<(String, Style)>) {
    let child_count = node.child_count();
    if child_count == 0 {
        let text = node.utf8_text(source.as_bytes()).unwrap_or("");
        let kind = node.kind();
        let style = if kind == "comment" || kind == "line_comment" || kind == "block_comment" {
            Style::default().fg(Color::Gray)
        } else if kind == "string_literal"
            || kind == "raw_string_literal"
            || kind == "character_literal"
        {
            Style::default().fg(Color::Green)
        } else if kind == "number_literal" || kind == "integer_literal" || kind == "float_literal" {
            Style::default().fg(Color::Cyan)
        } else if kind == "primitive_type" || kind == "type_identifier" || kind == "builtin_type" {
            Style::default().fg(Color::Blue)
        } else if [
            "fn",
            "let",
            "if",
            "else",
            "match",
            "return",
            "while",
            "for",
            "in",
            "struct",
            "enum",
            "impl",
            "pub",
            "use",
            "mod",
            "as",
            "type",
            "static",
            "const",
            "move",
            "unsafe",
            "extern",
            "crate",
            "class",
            "function",
            "var",
            "val",
            "def",
            "class",
            "public",
            "private",
            "protected",
            "import",
            "package",
            "func",
            "mut",
        ]
        .contains(&kind)
        {
            Style::default().fg(Color::Magenta)
        } else if kind == "macro_invocation" || kind == "attribute" || kind == "inner_attribute" {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        if !text.is_empty() {
            spans.push((text.to_string(), style));
        }
    } else {
        for i in 0..child_count {
            if let Some(child) = node.child(i as u32) {
                collect_styled_spans(&child, source, spans);
            }
        }
    }
}

pub struct TuiApp {
    pub original_content: String,
    pub new_content: String,
    pub editing: bool,
    pub accepted: bool,
    pub entities: Vec<String>,
    pub current_entity: usize,
    pub comments: Vec<String>,
    pub collecting_comment: bool,
    pub current_comment: String,
}

impl TuiApp {
    pub fn new(original: String, patched: String) -> Self {
        Self {
            original_content: original,
            new_content: patched,
            editing: false,
            accepted: false,
            entities: vec!["(no entities)".to_string()],
            current_entity: 0,
            comments: Vec::new(),
            collecting_comment: false,
            current_comment: String::new(),
        }
    }

    pub fn run(&mut self) -> Result<bool> {
        let hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
            hook(info);
        }));
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let res = self.run_app(&mut terminal);
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        res?;
        Ok(self.accepted)
    }

    fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if self.collecting_comment {
                        match key.code {
                            KeyCode::Enter => {
                                self.comments.push(self.current_comment.clone());
                                self.current_comment.clear();
                                self.collecting_comment = false;
                            }
                            KeyCode::Esc => {
                                self.current_comment.clear();
                                self.collecting_comment = false;
                            }
                            KeyCode::Char(c) => {
                                self.current_comment.push(c);
                            }
                            KeyCode::Backspace => {
                                self.current_comment.pop();
                            }
                            _ => {}
                        }
                        continue;
                    }
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            if self.editing {
                                self.editing = false;
                            } else {
                                self.accepted = true;
                                return Ok(());
                            }
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            if self.editing {
                                self.editing = false;
                            } else {
                                self.accepted = false;
                                return Ok(());
                            }
                        }
                        KeyCode::Char('e') | KeyCode::Char('E') => {
                            self.editing = !self.editing;
                        }
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            self.collecting_comment = true;
                            self.current_comment.clear();
                        }
                        KeyCode::Char('[') if !self.editing => {
                            if self.current_entity > 0 {
                                self.current_entity -= 1;
                            }
                        }
                        KeyCode::Char(']') if !self.editing => {
                            if self.current_entity + 1 < self.entities.len() {
                                self.current_entity += 1;
                            }
                        }
                        KeyCode::Tab => {}
                        KeyCode::Up | KeyCode::Down | KeyCode::PageUp | KeyCode::PageDown => {}
                        _ => {}
                    }
                }
            }
        }
    }

    fn ui(&self, f: &mut Frame) {
        let area = f.area();
        let vert = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Min(3), Constraint::Length(4)])
            .split(area);
        let top = vert[0];
        let bottom = vert[1];

        let horiz = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(top);

        let left_block = Block::default()
            .borders(Borders::ALL)
            .title(" Original ")
            .title_alignment(Alignment::Center);
        let left = Paragraph::new(self.original_content.as_str())
            .block(left_block)
            .wrap(Wrap { trim: true });
        f.render_widget(left, horiz[0]);

        let right_title = if self.collecting_comment {
            " Comment (Enter to save, Esc to cancel) "
        } else if self.editing {
            " Editing (e to toggle) "
        } else {
            " Patched "
        };
        let right_block = Block::default()
            .borders(Borders::ALL)
            .title(right_title)
            .title_alignment(Alignment::Center);
        let right = Paragraph::new(self.new_content.as_str())
            .block(right_block)
            .wrap(Wrap { trim: true });
        f.render_widget(right, horiz[1]);

        let help_first = Line::from(vec![
            Span::styled(
                " [Y] Accept ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " [N] Reject ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " [E] Edit  ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " [C] Comment ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(
                    " [[/]] Entity ({}/{}) ",
                    self.current_entity + 1,
                    self.entities.len()
                ),
                Style::default().fg(Color::White),
            ),
        ]);
        let mut help_lines = vec![help_first];
        if !self.comments.is_empty() {
            help_lines.push(Line::from(vec![Span::styled(
                format!(" Comments: {}", self.comments.join("; ")),
                Style::default().fg(Color::Cyan),
            )]));
        }
        let help = Paragraph::new(help_lines)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(help, bottom);
    }
}

pub fn show_diff(original: &str, patched: &str) -> Result<bool> {
    let mut app = TuiApp::new(original.to_string(), patched.to_string());
    app.run()
}
