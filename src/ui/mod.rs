mod content;
mod header;
mod layout;
mod player;
mod sidebar;
mod status;

use crate::app::App;
use ratatui::prelude::*;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let areas = layout::split(frame.area());
    header::render(frame, app, areas.header, areas.search);
    sidebar::render(frame, app, areas.sidebar);
    content::render(frame, app, areas.content);
    player::render(frame, app, areas.player);
}
