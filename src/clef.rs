use serde::Deserialize;
use serde_json::{self, Value};

#[derive(Deserialize, Debug, PartialEq)]
pub struct ClefLine {
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
}

impl ClefLine {
    pub fn new(line: &str) -> Self {
        let mut clef: ClefLine = serde_json::from_str(line).unwrap();
        clef.data = line.to_string();
        clef.render();
        clef
    }

    pub fn render(&self) -> String {
        let start_bracket = "{";
        let end_bracket = "}";
        let mut base = self.template.clone();
        let json:Value = serde_json::from_str(self.data.as_str()).unwrap();

        loop {
            let start = base.find(start_bracket).unwrap_or_default();
            let end = base.find(end_bracket).unwrap_or_default();
            if start == 0 {
                break;
            }
            let data_value = &base[start + 1..end];
            println!("{}", data_value);
            let json_value = &json[data_value];
            println!("{:?}", json_value);

            base.replace_range(start..end + 1, &json_value.to_string().as_str());
            println!("{}", base);
        }
        base
    }
}
