use crate::app::App;
use crate::ui::status;
use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let player_block = Block::default().title("Now Playing").borders(Borders::ALL);

    let inner = player_block.inner(area);
    frame.render_widget(player_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(4)])
        .split(inner);

    status::render(frame, app, chunks[0]);
    render_progress(frame, app, chunks[1]);
}

fn render_status(frame: &mut Frame, app: &App, area: Rect) {
    let playing = match &app.now_playing {
        Some(song) => format!("▶ {} - {}", song.artist, song.title),
        None => "▶ nothing".to_string(),
    };

    let status = Paragraph::new(playing);
    frame.render_widget(status, area);
}

fn render_progress(frame: &mut Frame, app: &App, area: Rect) {
    let (ratio, label) = calculate_progress(app);

    let gauge = Gauge::default().ratio(ratio).label(label);

    frame.render_widget(gauge, area);
}

fn calculate_progress(app: &App) -> (f64, String) {
    if let Some(song) = &app.now_playing {
        if song.duration > 0 {
            let ratio = (app.playback_seconds as f64 / song.duration as f64).clamp(0.0, 1.0);

            let label = format!(
                "{:02}:{:02}/{:02}:{:02}",
                app.playback_seconds / 60,
                app.playback_seconds % 60,
                song.duration / 60,
                song.duration % 60,
            );

            return (ratio, label);
        }
    }

    (0.0, "00:00/00:00".to_string())
}
