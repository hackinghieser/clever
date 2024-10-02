pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Application updater.
pub mod update;

pub mod event_log_level;

use std::{
    fs,
    io::{self},
};
use update::update;
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    file: Option<String>,
}

use app::App;
use clap::Parser;
use event::{Event, EventHandler};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{ListState, TableState},
    Terminal,
};
use tui::Tui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path: String;
    match args.file {
        Some(p) => path = p,
        None => {
            println!("No file path provided");
            return Ok(());
        }
    };
    // Create an application.
    let mut app = create_app(path)?;
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

fn create_app(path: String) -> Result<App<'static>, io::Error> {
    let lines = read_file(path.as_str())?;
    let mut app = App::new();
    app.file_path = path;
    app.event_table_state = TableState::new();
    app.filter_list_state = ListState::default();
    app.filter_list_state.select(Some(0));
    app.event_table_state.select(Some(0));
    app.load_lines(&lines);
    Ok(app)
}

fn read_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let content = fs::read_to_string(file_path)?;
    let mut lines: Vec<String> = vec![];
    for line in content.lines() {
        lines.push(line.to_string());
    }
    Ok(lines)
}
