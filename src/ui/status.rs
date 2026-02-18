use crate::app::{App, InputMode, View};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let status_line = build_status_line(app);

    let status = Paragraph::new(status_line).block(Block::default().borders(Borders::NONE));

    frame.render_widget(status, area);
}

fn build_status_line(app: &App) -> Line<'static> {
    let mode = match app.input_mode {
        InputMode::Normal => " NORMAL ",
        InputMode::Search => " SEARCH ",
    };

    let view = match app.current_view {
        View::Results => "RESULTS",
        View::Queue => "QUEUE",
    };

    let playing = match &app.now_playing {
        Some(song) => format!("▶ {} - {}", song.artist, song.title),
        None => "▶ nothing".to_string(),
    };

    let hint = match app.input_mode {
        InputMode::Normal => "",
        InputMode::Search => "  enter:confirm  esc:cancel",
    };

    Line::from(vec![
        Span::styled(
            mode,
            Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED),
        ),
        Span::raw(" "),
        Span::styled(view, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::raw(playing),
        Span::raw(" "),
        Span::styled(hint, Style::default().add_modifier(Modifier::DIM)),
    ])
}
