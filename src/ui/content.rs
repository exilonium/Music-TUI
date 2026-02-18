use crate::app::{App, View};
use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.current_view {
        View::Queue => render_queue(frame, app, area),
        View::Results => render_results(frame, area),
    }
}

fn render_queue(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .items
        .iter()
        .map(|i| ListItem::new(format!("{} - {}", i.artist, i.title)))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Music-TUI (1* Queue, 2- Results)")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_results(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Music-TUI (1- Queue, 2* Results)")
        .borders(Borders::ALL);

    frame.render_widget(block, area);
}
