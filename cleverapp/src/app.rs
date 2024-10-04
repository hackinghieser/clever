use crate::event_log_level::EventLogLevel;
use cleverlib::clef::ClefLine;
use ratatui::widgets::{ListState, Row, TableState};

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
    pub lines: Vec<ClefLine>,
    pub rows: Vec<Row<'a>>,
    pub event_table_state: TableState,
    pub filter_list_state: ListState,
    pub file_path: String,
    pub event_types: Vec<EventLogLevel>,
    pub app_state: AppState,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn load_lines(&mut self, lines: &Vec<String>) {
        self.lines = self.create_cells_from_line(lines);
    }

    fn create_cells_from_line(&mut self, lines: &Vec<String>) -> Vec<ClefLine> {
        let mut clef_lines: Vec<ClefLine> = vec![];

        for line in lines {
            clef_lines.push(ClefLine::new(line).unwrap())
        }
        self.get_event_types(&clef_lines);
        clef_lines
    }

    pub fn get_event_types(&mut self, events: &Vec<ClefLine>) {
        for event in events {
            if !self.event_types.iter().any(|t| t.value == event.level) {
                self.event_types.push(EventLogLevel {
                    value: event.level.to_string(),
                    selected: true,
                });
            }
        }
        self.event_types
            .iter()
            .for_each(|v| println!("{}", v.value));
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn move_row_up(&mut self, range: usize) {
        if let Some(selected) = self.event_table_state.selected() {
            if selected >= range + 1 {
                self.event_table_state.select(Some(selected - range));
            } else {
                self.event_table_state.select(Some(self.lines.len() - 1));
            }
        }
    }

    pub fn move_row_down(&mut self, range: usize) {
        if let Some(selected) = self.event_table_state.selected() {
            if selected < self.lines.len() - range {
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
            if selected < self.event_types.len() {
                self.filter_list_state.select(Some(selected + 1));
            } else {
                self.filter_list_state.select(Some(0));
            }
        }
    }
}
