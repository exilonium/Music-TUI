mod app;
mod ui;
use app::App;

use ratatui::prelude::*;
use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        // input handling
        #[allow(clippy::collapsible_if)]
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    app::InputMode::Normal => match key.code {
                        KeyCode::Char('/') => app.input_mode = app::InputMode::Search,
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') | KeyCode::Down => app.next(),

                        KeyCode::Char('k') | KeyCode::Up => app.prev(),
                        _ => (),
                    },
                    app::InputMode::Search => match key.code {
                        KeyCode::Esc => app.input_mode = app::InputMode::Normal,
                        KeyCode::Char(c) => app.input.push(c),
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        _ => {}
                    },
                }
            }
        }
    }
    // std::thread::sleep(std::time::Duration::from_secs(3));
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
