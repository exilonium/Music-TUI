use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let items = vec!["coffee", "Ghost", "Understand", "CHIHIRO"];
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            let items: Vec<ListItem> = items.iter().map(|i| ListItem::new(*i)).collect();

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

            frame.render_stateful_widget(list, chunks[0], &mut list_state.clone());

            let status = Block::default().title("Now Playing").borders(Borders::ALL);
            frame.render_widget(status, chunks[1]);
        })?;

        // input handling
        #[allow(clippy::collapsible_if)]
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') | KeyCode::Down => {
                        let i = match list_state.selected() {
                            Some(i) => {
                                if i >= items.len() - 1 {
                                    i
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }

                    KeyCode::Char('k') | KeyCode::Up => {
                        let i = match list_state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    0
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        list_state.select(Some(i));
                    }
                    _ => {}
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

