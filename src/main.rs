use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

struct App<'a> {
    items: Vec<&'a str>,
    list_state: ratatui::widgets::ListState,
}

impl<'a> App<'a> {
    fn new() -> Self {
        let items = vec!["coffee", "Ghost", "Understand", "CHIHIRO"];
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self { items, list_state }
    }
    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
    fn prev(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let items: Vec<ListItem> = app.items.iter().map(|i| ListItem::new(*i)).collect();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(area);

            let list = List::new(items)
                .block(
                    Block::default()
                        .title("Music-TUI - j/k to move, q to quit")
                        .borders(Borders::all()),
                )
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ");

            frame.render_stateful_widget(list, chunks[0], &mut app.list_state.clone());

            let selected_test = match app.list_state.selected() {
                Some(i) => format!("Selected: {}", app.items[i]),
                None => "Selected Nothing".to_string(),
            };
            let status = Paragraph::new(selected_test)
                .block(Block::default().title("Now Playing").borders(Borders::ALL));

            frame.render_widget(status, chunks[1]);
        })?;

        // input handling
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') | KeyCode::Down => app.next(),

                    KeyCode::Char('k') | KeyCode::Up => app.prev(),
                    _ => (),
                }
            }
        }
    }
    // std::thread::sleep(std::time::Duration::from_secs(3));
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

