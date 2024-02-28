#[derive(Debug, PartialEq)]
pub struct EventLogLevel {
    pub selected: bool,
    pub value: String,
}

impl ToString for EventLogLevel {
    fn to_string(&self) -> String {
        if !self.selected {
            self.value.to_string()
        } else {
            format!("* {}", self.value)
        }
    }
}
