use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        KeyCode::Right | KeyCode::Char('j') => app.move_row_down(10),
        KeyCode::Left | KeyCode::Char('k') => app.move_row_up(10),
        KeyCode::Up | KeyCode::Char('h') => app.move_row_up(1),
        KeyCode::Down | KeyCode::Char('l') => app.move_row_down(1),
        _ => {}
    };
}