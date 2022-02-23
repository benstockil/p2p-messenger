mod actors;
mod actor;
mod objects;
mod remote;

use crate::actors::StateHandler;
use crate::actors::PeerListener;

mod ui;

use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Widget};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use crossterm::{self, execute};

#[derive(Debug)]
struct App {
    messages: Vec<String>,
    input: String,
    groups: Vec<String>,
    selected_group: usize,
}


impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            messages: Vec::new(),
            groups: Vec::new(),
            selected_group: 0,
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> { 
    // let state_handler = StateHandler::new();
    // let ui_handler = UiHandler::new(state_handler.clone());
    // let listener = PeerListener::new(state_handler.clone());

    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(
        stdout, 
        EnterAlternateScreen, 
        EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;    
 
    let mut app = App::default();
    app.groups = vec![
        "Group 1".into(),
        "Group 2".into(),
        "Group 3".into(),
        "Group 4".into(),
    ];

    let mut run = true;
    while run {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(30),
                    Constraint::Min(1),
                ])
                .split(frame.size());

            let communication_pane = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1), 
                    Constraint::Length(3)
                ])
                .split(layout[1]);

            let message_items: Vec<ListItem> = app
                .messages
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let content = vec![Span::raw(format!("{}: {}", i, m)).into()];
                    ListItem::new(content)
                })
                .collect();

            let messages = List::new(message_items)
                .block(Block::default().borders(Borders::ALL).title("Messages"));
            frame.render_widget(messages, communication_pane[0]);

            let input = Paragraph::new(app.input.as_ref())
                .block(Block::default().borders(Borders::ALL).title("Input"));
            frame.render_widget(input, communication_pane[1]);

            frame.set_cursor(
                communication_pane[1].x + 1 + app.input.len() as u16, 
                communication_pane[1].y + 1
            );

            let group_items: Vec<ListItem> = app
                .groups
                .iter()
                .enumerate()
                .map(|(i, group)| {
                    let item = ListItem::new(vec![Span::raw(group).into()]);
                    let style = if i == app.selected_group {
                        Style::default().fg(Color::Black).bg(Color::Blue).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    item.style(style)
                })
                .collect();
            let groups = List::new(group_items)
                .block(Block::default().borders(Borders::ALL).title("Groups"));
            frame.render_widget(groups, layout[0]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    app.input.push(c);
                },
                KeyCode::Backspace => {
                    app.input.pop();
                },
                KeyCode::Enter => {
                    app.messages.push(app.input.drain(..).collect())
                }
                KeyCode::Tab => {
                    app.selected_group = (app.selected_group + 1) % app.groups.len()
                }
                KeyCode::End => {
                    run = false;
                }
                _ => {}
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    
    Ok(())
}
