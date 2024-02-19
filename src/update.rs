use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, AppState};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.app_state {
        AppState::ITERATING => {
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
                KeyCode::Char('f') | KeyCode::Char('F') => match app.app_state {
                    AppState::ITERATING => app.app_state = AppState::FILTERING,
                    AppState::FILTERING => app.app_state = AppState::ITERATING,
                },
                _ => {}
            };
        }
        AppState::FILTERING => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit()
                }
            }
            KeyCode::Right | KeyCode::Char('j') => app.move_list_up(),
            KeyCode::Left | KeyCode::Char('k') => app.move_list_down(),
            KeyCode::Up | KeyCode::Char('h') => app.move_list_up(),
            KeyCode::Down | KeyCode::Char('l') => app.move_list_down(),
            KeyCode::Enter | KeyCode::Char(' ') =>  {
                let selected = app.filter_list_state.selected().unwrap();
                app.event_types[selected].selected = !app.event_types[selected].selected;
            } 
            KeyCode::Char('f') | KeyCode::Char('F') => match app.app_state {
                AppState::ITERATING => app.app_state = AppState::FILTERING,
                AppState::FILTERING => app.app_state = AppState::ITERATING,
            },
            _ => {}
        },
    }
}
