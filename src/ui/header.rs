use crate::app::{App, InputMode};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, app: &App, header_area: Rect, search_area: Rect) {
    render_title(frame, header_area);
    render_search(frame, app, search_area);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let header = Block::default()
        .title("Music-TUI")
        .title_alignment(Alignment::Center);

    frame.render_widget(header, area);
}

fn render_search(frame: &mut Frame, app: &App, area: Rect) {
    let search = Paragraph::new(app.input.as_str())
        .block(Block::default().title("Search").borders(Borders::ALL));

    frame.render_widget(search, area);

    // Cursor handling (only when searching)
    if let InputMode::Search = app.input_mode {
        frame.set_cursor_position((area.x + 1 + app.input.len() as u16, area.y + 1));
    }
}
