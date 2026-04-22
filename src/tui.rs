use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::panic;

pub struct TuiApp {
    diff_content: String,
    accepted: bool,
}

impl TuiApp {
    pub fn new(diff_content: String) -> Self {
        Self {
            diff_content,
            accepted: false,
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
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            self.accepted = true;
                            return Ok(());
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            self.accepted = false;
                            return Ok(());
                        }
                        KeyCode::Up | KeyCode::Down | KeyCode::PageUp | KeyCode::PageDown => {
                            // Scrolling handled automatically by Paragraph's wrap
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
            .split(f.area());

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Patch Diff ")
            .title_alignment(Alignment::Center);
        let paragraph = Paragraph::new(self.diff_content.clone())
            .block(block)
            .wrap(Wrap { trim: true })
            .scroll((0, 0));
        f.render_widget(paragraph, chunks[0]);

        let help_text = vec![
            Line::from(vec![
                Span::styled("[Y]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" Accept  "),
                Span::styled("[N]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" Reject  "),
                Span::styled("[ESC]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" Quit  "),
            ]),
        ];
        let help = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(help, chunks[1]);
    }
}

pub fn show_diff(diff: &str) -> Result<bool> {
    let mut app = TuiApp::new(diff.to_string());
    app.run()
}
