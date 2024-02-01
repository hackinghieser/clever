use ratatui::widgets::{Cell, Row};
use serde::Deserialize;
use serde_json::{self, Value};

#[derive(Deserialize, Debug, PartialEq)]
pub struct ClefLine<'a> {
    #[serde(rename = "@t")]
    #[serde(default)]
    time: String,

    #[serde(rename = "@m")]
    #[serde(default)]
    message: String,

    #[serde(rename = "@mt")]
    #[serde(default)]
    template: String,

    #[serde(rename = "@l")]
    #[serde(default)]
    level: String,

    #[serde(rename = "@x")]
    #[serde(default)]
    exception: String,

    #[serde(rename = "@i")]
    #[serde(default)]
    eventid: String,

    #[serde(rename = "@r")]
    #[serde(default)]
    renderings: String,

    #[serde(skip)]
    data: String,

    #[serde(skip)]
    pub row: Row<'a>,
}

impl<'a> ClefLine<'a> {

    pub fn new(line: &str) -> Self {
        let mut clef: ClefLine = serde_json::from_str(line).unwrap();
        clef.data = line.to_string();
        clef.template = clef.render();
        println!("{}", clef.template);
        clef.row = Row::new(vec![
            Cell::from(clef.time.to_string()),
            Cell::from(clef.template.to_string()),
        ]);
        clef
    }

    pub fn render(&mut self) -> String {
        let start_bracket = "{";
        let end_bracket = "}";
        let mut base = self.template.clone();
        let json: Value = serde_json::from_str(self.data.as_str()).unwrap();
        println!("JSON {}", base.to_string());
        loop {
            let start = base.find(start_bracket).unwrap_or_default();
            let end = base.find(end_bracket).unwrap_or_default();
            println!("START {}", start);
            if end == 0 {
                break;
            }
            let data_value = &base[start + 1..end];
            let json_value = &json[data_value];
            let mut template_value: String = String::from("empty");
            if let Some(number) = json_value.as_i64() {
                template_value = number.to_string();
            }

            if let Some(string) = json_value.as_str() {
                template_value = string.to_string();
            }
            println!("JSON VALUE {}", template_value.to_string());
            base.replace_range(start..end + 1, template_value.as_str());
        }
   
        base
    }
}
