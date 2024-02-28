use std::default;

use super::{clef_event::ClefEvent, event_log_level::EventLogLevel};

#[derive(Debug,PartialEq)]
pub struct ClefController<'a> {
    pub lines: Vec<ClefEvent<'a>>,
    pub event_types: Vec<EventLogLevel>,
}

impl<'a> Default for ClefController<'a> {
    fn default() -> Self {
        Self { lines: Default::default(), event_types: Default::default() }
    }
}

impl<'a> ClefController<'a> {
    pub fn load_lines(&mut self, lines: &Vec<String>) {
        self.lines = self.create_cells_from_line(lines);
    }

    fn create_cells_from_line(&mut self, lines: &Vec<String>) -> Vec<ClefEvent<'a>> {
        let mut clef_lines: Vec<ClefEvent<'_>> = vec![];

        for line in lines {
            clef_lines.push(ClefEvent::new(line).unwrap())
        }
        self.get_event_types(&clef_lines);
        clef_lines
    }

    pub fn get_event_types(&mut self, events: &Vec<ClefEvent>) {
        for event in events {
            if !self.event_types.iter().any(|t| t.value == event.level) {
                self.event_types.push(EventLogLevel {
                    selected: true,
                    value: event.level.to_string(),
                });
            }
        }
        self.event_types
            .iter()
            .for_each(|v| println!("{}", v.value));
    }
}
