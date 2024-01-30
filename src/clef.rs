use serde::Deserialize;
use serde_json;

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
}

impl ClefLine {
    
    pub fn new(line: &str) -> Self {
        let clef: ClefLine = serde_json::from_str(line).unwrap();
        clef
    }

    pub fn render(&self) -> String {
        !unimplemented!()
    }
}
