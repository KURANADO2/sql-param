mod app;
mod core;
mod event_handler;
mod log_parser;
mod ui;

use crate::app::App;
use crate::event_handler::{handle_key, handle_mouse, handle_paste};
use crate::ui::ui;
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, Event,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        EnableBracketedPaste
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        DisableBracketedPaste
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>> {
    while !app.should_exit {
        // parse clipboard first
        app.parse_clipboard();
        // draw ui
        terminal.draw(|f| ui(f, app))?;
        if let Ok(event) = event::read() {
            match event {
                Event::Key(key) => handle_key(app, key),
                Event::Paste(data) => handle_paste(app, data),
                Event::Mouse(mouse) => handle_mouse(app, mouse),
                _ => {}
            }
        };
    }
    Ok(())
}
