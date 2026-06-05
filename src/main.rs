use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::*};
use std::io;

mod app;
mod ui;

use app::App;

use crate::app::AppMode;

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run
    let result = run(&mut terminal);

    // Teardown — TOUJOURS restaurer le terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            // q quitte depuis n'importe quel mode sauf Popup
            if key.code == KeyCode::Char('q') && app.mode != AppMode::Popup {
                break;
            }
            app.handle_key(key.code);
        }
    }
    Ok(())
}
