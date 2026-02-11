use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::{App, InputMode, View};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    match app.current_view {
        View::Queue => {
            let items: Vec<ListItem> = app.items.iter().map(|i| ListItem::new(*i)).collect();
            let list = List::new(items)
                .block(
                    Block::default()
                        .title("Music-TUI (1* Queue, 2- Results | - j/k to move, q to quit")
                        .borders(Borders::all()),
                )
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ");
            frame.render_stateful_widget(list, chunks[0], &mut app.list_state.clone());
        }
        View::Results => {
            let block = Block::default()
                .title("Music-TUI (1- Queue, 2* Results | - j/k to move, q to quit")
                .borders(Borders::all())
                .borders(Borders::ALL);
            frame.render_widget(block, chunks[0]);
        }
    }

    let bottom_text = match app.input_mode {
        InputMode::Normal => match app.now_playing {
            Some(song) => format!("Now Playing: {}", song),
            None => "Playing Nothing".to_string(),
        },
        InputMode::Search => format!("Search: {}", app.input),
    };

    let status = Paragraph::new(bottom_text)
        .block(Block::default().title("Now Playing").borders(Borders::ALL));

    frame.render_widget(status, chunks[1]);
}
