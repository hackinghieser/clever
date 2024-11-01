use indexmap::IndexMap;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Event {
    #[serde(rename = "@t")]
    #[serde(default)]
    pub time: Option<String>,

    #[serde(rename = "@m")]
    #[serde(default)]
    pub message: Option<String>,

    #[serde(rename = "@mt")]
    #[serde(default)]
    pub template: String,

    #[serde(rename = "@l")]
    #[serde(default)]
    pub level: Option<String>,

    #[serde(rename = "@x")]
    #[serde(default)]
    pub exception: Option<String>,

    #[serde(rename = "@i")]
    #[serde(default)]
    pub eventid: Option<String>,

    #[serde(rename = "@r")]
    #[serde(default)]
    pub renderings: Vec<String>,

    #[serde(skip)]
    pub data: String,

    #[serde(flatten)]
    properties: IndexMap<String, Value>,

    #[serde(skip)]
    raw_string: String,
}

impl Event {
    pub fn create(raw_event: String, regex: &Regex) -> Option<Self> {
        let raw_json: Value = serde_json::from_str(raw_event.as_str()).unwrap();
        let mut event: Event = serde_json::from_value(raw_json.clone()).unwrap();
        event.raw_string = raw_event;
        event.message = Some(Event::generate_message_template(
            &event.template,
            &event.properties,
            regex,
        ));
        Some(event)
    }

    //TODO: Parse format and alignment as well using regex matches
    fn generate_message_template(
        template: &str,
        properties: &IndexMap<String, Value>,
        regex: &Regex,
    ) -> String {
        regex
            .replace_all(template, |caps: &regex::Captures| {
                let key = &caps[1];
                if let Ok(index) = key.parse::<usize>() {
                    properties
                        .get_index(index)
                        .map_or(format!("{{{}}}", index), |v| {
                            println!("Index Value: {}", v.1);
                            v.1.to_string()
                        })
                } else {
                    properties
                        .get(key)
                        .map_or(format!("{{{}}}", key), |v| v.to_string())
                }
            })
            .to_string()
    }
}
