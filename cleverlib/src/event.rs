use regex::Regex;
use serde::Deserialize;

use crate::{template, token::Token};

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
    pub template: Option<String>,

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

    #[serde(skip)]
    pub tokens: Vec<Token>,

    #[serde(skip)]
    raw_string: String,
}

impl Event {
    pub fn create(raw_event: String) -> Option<Self> {
        let mut event: Event = serde_json::from_str(raw_event.as_str()).unwrap();
        event.raw_string = raw_event;
        event.tokenize();
        Some(event)
    }

    fn tokenize(&mut self) {
        let mut tokens: Vec<Token> = Vec::new();
        let template = &self.template.as_ref().unwrap();
        let splitted: Vec<&str> = template.split_whitespace().collect();
        println!("Event entities: {}", splitted.len());
        println!("Event entities: {:?}", splitted);
        splitted.iter().for_each(|x| {
            println!("Token: {}", x);
            let t = Token::new(x.to_string(), self.raw_string.to_string());
            tokens.push(t.unwrap())
        });
        self.tokens = tokens;
    }
}
