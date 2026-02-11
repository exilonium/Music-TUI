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
                        .title("Music-TUI (1* Queue, 2- Results")
                        .borders(Borders::all()),
                )
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ");
            frame.render_stateful_widget(list, chunks[0], &mut app.list_state.clone());
        }
        View::Results => {
            let block = Block::default()
                .title("Music-TUI (1- Queue, 2* Results")
                .borders(Borders::all())
                .borders(Borders::ALL);
            frame.render_widget(block, chunks[0]);
        }
    }
    let mode_text = match app.input_mode {
        InputMode::Normal => "Normal",
        InputMode::Search => "Search",
    };
    let view_text = match app.current_view {
        View::Results => "Results",
        View::Queue => "Queue",
    };
    let hints = match app.input_mode {
        InputMode::Normal => "j/k move • / search • Enter play • 1/2 switch view • q quit",
        InputMode::Search => "Type to search • Enter submit • Esc cancel",
    };
    let playing_text = match app.now_playing {
        Some(song) => format!("🎧 {} [{}s]", song, app.playback_seconds),
        None => "no song yapping".to_string(),
    };
    let bottom_text = format!(
        "{} | {} | {} | {}",
        mode_text, view_text, playing_text, hints
    );

    let status = Paragraph::new(bottom_text)
        .block(Block::default().title("Music-TUI").borders(Borders::ALL));

    frame.render_widget(status, chunks[1]);
}
