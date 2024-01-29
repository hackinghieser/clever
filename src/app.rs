use ratatui::widgets::{Cell, Row, TableState};
use chrono::{format::Fixed, prelude::*};
#[derive(Debug, Default)]
pub struct App<'a> {
    pub should_quit: bool,
    pub counter: u8,
    pub lines: Vec<String>,
    pub rows: Vec<Row<'a>>,
    pub table_state: TableState,
}


impl<'a> App<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn load_lines(&mut self, lines: &Vec<String>) {
        self.rows = Self::create_cells_from_line(lines);
    }

    fn create_cells_from_line(lines: &Vec<String>) -> Vec<Row<'a>> {
        let mut rows: Vec<Row<'_>> = vec![];
        let time  = DateTime::parse_from_str("2014-11-28 21:00:09 +09:00","%Y-%m-%d %H:%M:%S %z");
        let time = time.unwrap().format("%d.%m.%y %H:%M:%S");
        for (index, line) in lines.iter().enumerate() {
            let row = Row::new(vec![
                Cell::from(index.to_string()),
                Cell::from(time.to_string()),
                Cell::from(line.to_string()),
            ]);
            rows.push(row)
        }
        rows
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn move_row_up(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            if selected > 1 {
                self.table_state.select(Some(selected - 1));
            } else {
                self.table_state.select(Some(self.rows.len() - 1));
            }
        }
    }

    pub fn move_row_down(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            if selected < self.rows.len() - 1 {
                self.table_state.select(Some(selected + 1));
            } else {
                self.table_state.select(Some(0));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0)
    }
}
