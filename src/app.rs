use ratatui::widgets::TableState;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub counter: u8,
    pub lines: Vec<String>,
    pub table_state: TableState,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

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
            if  selected >= 1 {
                self.table_state.select(Some(selected - 1));
            }
        }
    }

    pub fn move_row_down(&mut self) {
        if let Some(selected) = self.table_state.selected() {
            if selected < self.lines.len()-1 {
                self.table_state.select(Some(selected + 1));
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
