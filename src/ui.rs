use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
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
}
