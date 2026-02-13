mod api;
mod app;
mod ui;
use app::{Action, App};

use ratatui::prelude::*;
use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crate::api::songs::search_and_get_url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                let action = match key.code {
                    KeyCode::Char('/') => Action::EnterSearch,
                    KeyCode::Char('q') => Action::Quit,
                    KeyCode::Char('j') | KeyCode::Down => Action::Down,
                    KeyCode::Char('k') | KeyCode::Up => Action::Up,
                    KeyCode::Char('1') => Action::SwitchQueueView,
                    KeyCode::Char('2') => Action::SwitchResultView,
                    KeyCode::Esc => Action::ExitSearch,
                    KeyCode::Enter => Action::SubmitSearch,
                    KeyCode::Char(c) => Action::InputChar(c),
                    KeyCode::Backspace => Action::BackSpace,
                    _ => Action::None,
                };
                if app.handle_action(action).await {
                    break;
                }
            }
        } else {
            app.handle_action(Action::Tick).await;
        }
    }
    // std::thread::sleep(std::time::Duration::from_secs(3));
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
