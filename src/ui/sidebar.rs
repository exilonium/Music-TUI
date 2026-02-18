use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, _app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)])
        .split(area);

    let lib_items = vec!["Tracks", "Albums", "Artists", "Play Queue"];
    let lib = List::new(lib_items).block(Block::default().title("Library").borders(Borders::ALL));

    frame.render_widget(lib, chunks[0]);

    let playlists = Block::default().title("Playlists").borders(Borders::ALL);

    frame.render_widget(playlists, chunks[1]);
}
