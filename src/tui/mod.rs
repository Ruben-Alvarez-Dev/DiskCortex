//! TUI Module

mod app;
mod ui;
mod widgets;

pub use app::*;
pub use ui::*;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

/// Run the TUI application
pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) | (_, KeyCode::Char('q')) => return Ok(()),
                    (_, KeyCode::Up) => app.prev_item(),
                    (_, KeyCode::Down) => app.next_item(),
                    (_, KeyCode::Left) => app.prev_tab(),
                    (_, KeyCode::Right) => app.next_tab(),
                    (_, KeyCode::Char('s')) => app.scan(),
                    (_, KeyCode::Char(' ')) => app.toggle_select(),
                    (_, KeyCode::Enter) => app.toggle_select(),
                    (_, KeyCode::Char('a')) => app.select_all(),
                    (_, KeyCode::Char('d')) => app.deselect_all(),
                    (_, KeyCode::Char('1')) => app.view = ViewMode::Overview,
                    (_, KeyCode::Char('2')) => app.view = ViewMode::Tools,
                    (_, KeyCode::Char('3')) => app.view = ViewMode::Cleanup,
                    (_, KeyCode::Char('4')) => app.view = ViewMode::Settings,
                    (_, KeyCode::Char('?')) => app.show_help = !app.show_help,
                    _ => {}
                }
            }
        }

        app.tick();
    }
}
