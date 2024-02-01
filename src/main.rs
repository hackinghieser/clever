/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

// clef parser
pub mod clef;

use std::fs;
use app::App;
use clef::ClefLine;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, widgets::TableState, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an application.
    let mut app = create_app();
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }
    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

fn create_app() -> App<'static> {
    let lines = read_file("src/example.clef");
    let mut app = App::new();
    app.table_state = TableState::new();
    app.table_state.select(Some(0));
    app.load_lines(&lines);
    app
}

fn read_file(file_path: &str) -> Vec<String> {
  let content = fs::read_to_string(file_path).unwrap();
  let mut lines : Vec<String> = vec![];
  for line in content.lines() {
      lines.push(line.to_string());
  }
  lines
}