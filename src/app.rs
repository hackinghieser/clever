use ratatui::widgets::{Row, TableState};

use crate::clef::ClefLine;
#[derive(Debug, Default)]
pub struct App<'a> {
    pub should_quit: bool,
    pub counter: u8,
    pub lines: Vec<ClefLine<'a>>,
    pub rows: Vec<Row<'a>>,
    pub table_state: TableState,
    pub file_path: String
}


impl<'a> App<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn load_lines(&mut self, lines: &Vec<String>) {
        self.lines = Self::create_cells_from_line(lines);
    }

    fn create_cells_from_line(lines: &Vec<String>) -> Vec<ClefLine<'a>> {
        let mut clef_lines: Vec<ClefLine<'_>> = vec![];
      
        for line in lines {
            clef_lines.push(ClefLine::new(line).unwrap())
        }
        clef_lines
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn move_row_up(&mut self, range: usize) {
        if let Some(selected) = self.table_state.selected() {
            if selected >= range {
                self.table_state.select(Some(selected - range));
            } else {
                self.table_state.select(Some(self.lines.len() - 1));
            }
        }
    }

    pub fn move_row_down(&mut self, range: usize) {
        if let Some(selected) = self.table_state.selected() {
            if selected < self.lines.len() - range {
                self.table_state.select(Some(selected + range));
            } else {
                self.table_state.select(Some(0));
            }
        }
    }
}
