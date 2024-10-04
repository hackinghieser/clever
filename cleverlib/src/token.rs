use regex::Regex;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub raw_token: String,
    pub token: Option<String>,
    pub arguments: Option<Value>,
    pub value: Option<String>,
}

impl Token {
    pub fn new(token: String, args: String) -> Option<Self> {
        let mut token = Token {
            raw_token: token,
            arguments: Some(serde_json::from_str(args.as_str()).unwrap()),
            value: None,
            token: None,
        };
        token.value = Some(Token::get_value(&token).to_string());
        Some(token)
    }

    fn clean(&self) -> String {
        let cleaned = self.raw_token.replace("{", "");
        let cleaned = cleaned.replace("}", "");
        let cleaned = cleaned.trim();
        cleaned.to_string()
    }

    pub fn render(&self) -> String {
        let rendered_value = &self.value.clone().unwrap().to_string();
        rendered_value.to_string()
    }

    fn get_value(&self) -> String {
        // Regex Match different kinds of holes
        // {} {{}}
        let split: Vec<&str> = self.raw_token.split(['{', '}']).collect();
        if split.len() > 1 {
            println!("RegexSplit: {:?}", split);
            let json = &self.arguments.as_ref().unwrap()[split.get(1).unwrap()];
            let s = format!("{}{}{}", split[0], json.as_str().unwrap(), split[2]);
            s
        } else {
            self.raw_token.to_string()
        }
    }
}
