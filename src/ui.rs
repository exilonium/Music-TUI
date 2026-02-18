use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};

use crate::app::{App, InputMode, View};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(3), // Search
            Constraint::Min(10),   // Main content grows
            Constraint::Length(6), // Playing panel bottom
        ])
        .split(area);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)])
        .split(root[2]);

    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Library
            Constraint::Min(0),    // Playlists
        ])
        .split(main_chunks[0]);

    let player_block = Block::default().title("Now Playing").borders(Borders::ALL);

    let inner = player_block.inner(root[3]);

    frame.render_widget(player_block, root[3]);
    let player_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(4)])
        .split(inner);

    let header = Block::default()
        .title("Music-TUI")
        .title_alignment(Alignment::Center);
    frame.render_widget(header, root[0]);

    let search = Paragraph::new(app.input.as_str())
        .block(Block::default().title("Search").borders(Borders::ALL));

    let search_area = root[1];
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Search => {
            frame.set_cursor_position((
                search_area.x + 1 + app.input.len() as u16,
                search_area.y + 1,
            ));
        }
    }

    frame.render_widget(search, root[1]);
    let test_lib = vec!["Tracks", "Albums", "Artists", "Play Queue"];
    let lib = List::new(test_lib).block(Block::default().title("Library").borders(Borders::ALL));
    frame.render_widget(lib, sidebar_chunks[0]);

    let playlists = Block::default().title("Playlists").borders(Borders::ALL);
    frame.render_widget(playlists, sidebar_chunks[1]);
    match app.current_view {
        View::Queue => {
            let items: Vec<ListItem> = app
                .items
                .iter()
                .map(|i| ListItem::new(format!("{} - {}", i.artist, i.title)))
                .collect();
            let list = List::new(items)
                .block(
                    Block::default()
                        .title("Music-TUI (1* Queue, 2- Results)")
                        .borders(Borders::all()),
                )
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .highlight_symbol(">> ");
            frame.render_stateful_widget(list, main_chunks[1], &mut app.list_state.clone());
        }
        View::Results => {
            let block = Block::default()
                .title("Music-TUI (1- Queue, 2* Results)")
                .borders(Borders::ALL);
            frame.render_widget(block, main_chunks[1]);
        }
    }

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

    // show hints ONLY when searching (very vim)
    let hint = match app.input_mode {
        InputMode::Normal => "",
        InputMode::Search => "  enter:confirm  esc:cancel",
    };

    let status_line = Line::from(vec![
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
    ]);

    let status = Paragraph::new(status_line).block(Block::default().borders(Borders::NONE));

    frame.render_widget(status, player_chunk[0]);
    let (progress, label) = if let Some(song) = &app.now_playing {
        let duration = song.duration;

        if duration > 0 {
            let ratio = (app.playback_seconds as f64 / duration as f64).clamp(0.0, 1.0);

            let label = format!(
                "{:02}:{:02}/{:02}:{:02}",
                app.playback_seconds / 60,
                app.playback_seconds % 60,
                duration / 60,
                duration % 60,
            );

            (ratio, label)
        } else {
            (0.0, "00:00/00:00".to_string())
        }
    } else {
        (0.0, "00:00/00:00".to_string())
    };

    let seek_bar = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(progress)
        .label(label);

    frame.render_widget(seek_bar, player_chunk[1]);
}
