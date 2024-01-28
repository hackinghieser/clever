use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use std::io::{stdout, Result};

pub mod app;
struct App {
    counter: i8,
    should_quit: bool,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "
            Press 'q' or 'Q' to exit the App. \n\
            Press 'j' or 'k' to increase/decrease the counter \n\
            Counter {}
            ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(ratatui::layout::Alignment::Center)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        )
        .style(Style::default().fg(Color::LightYellow))
        .alignment(ratatui::layout::Alignment::Center),
        f.size(),
    );
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(50)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('j') => {
                        if let Some(res) = app.counter.checked_add(1) {
                            app.counter = res;
                        } else {
                            app.counter = 0;
                        }
                    }
                    KeyCode::Char('k') => {
                        if let Some(res) = app.counter.checked_sub(1) {
                            app.counter = res;
                        } else {
                            app.counter = 0;
                        }
                    }
                    KeyCode::Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    let mut app = App {
        counter: 0,
        should_quit: false,
    };
    loop {
        terminal.draw(|f| ui(&app, f))?;
        update(&mut app)?;
        if app.should_quit {
            break;
        }
    };
    Ok(())
}
fn main() -> Result<()> {
    startup()?;
    let status = run();
    shutdown()?;
    status?;
    Ok(())
}
