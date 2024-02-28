use ratatui::widgets::{ListState, Row, TableState};

use crate::clef_events::clef_controller::ClefController;

#[derive(Debug)]
pub enum AppState {
    FILTERING,
    ITERATING,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::ITERATING
    }
}
#[derive(Debug, Default)]
pub struct App<'a> {
    pub should_quit: bool,
    pub counter: u8,
    pub rows: Vec<Row<'a>>,
    pub event_table_state: TableState,
    pub filter_list_state: ListState,
    pub file_path: String,
    pub app_state: AppState,
    pub clef_controller: ClefController<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn move_row_up(&mut self, range: usize) {
        if let Some(selected) = self.event_table_state.selected() {
            if selected >= range + 1 {
                self.event_table_state.select(Some(selected - range));
            } else {
                self.event_table_state.select(Some(self.clef_controller.lines.len() - 1));
            }
        }
    }

    pub fn move_row_down(&mut self, range: usize) {
        if let Some(selected) = self.event_table_state.selected() {
            if selected < self.clef_controller.lines.len() - range {
                self.event_table_state.select(Some(selected + range));
            } else {
                self.event_table_state.select(Some(0));
            }
        }
    }

    pub fn move_list_up(&mut self) {
        if let Some(selected) = self.filter_list_state.selected() {
            if selected >= 1 {
                self.filter_list_state.select(Some(selected - 1));
            } else {
                self.filter_list_state.select(Some(0));
            }
        }
    }

    pub fn move_list_down(&mut self) {
        if let Some(selected) = self.filter_list_state.selected() {
            if selected < self.clef_controller.event_types.len() {
                self.filter_list_state.select(Some(selected + 1));
            } else {
                self.filter_list_state.select(Some(0));
            }
        }
    }
}
