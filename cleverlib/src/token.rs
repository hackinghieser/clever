use serde_json::Value;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub raw_token: String,
    pub token: Option<String>,
    pub arguments: Option<Value>,
    pub value: Option<String>,
}

impl Token {
    pub fn new(token: String, event: &Value) -> Option<Self> {
        let mut token = Token {
            raw_token: token,
            arguments: Some(serde_json::from_value(event.clone()).unwrap()),
            value: None,
            token: None,
        };
        (token.value, token.token) = token.parse();
        Some(token)
    }

    pub fn render(&self) -> String {
        let rendered_value = &self.value.clone().unwrap().to_string();
        rendered_value.to_string()
    }

    fn parse(&self) -> (Option<String>, Option<String>) {
        // Regex Match different kinds of holes
        // {} {{}}
        let mut value = self.raw_token.to_string();
        println!("{:?}", self.arguments);
        let split_token: Vec<&str> = self.raw_token.split(['{', '}']).collect();
        if split_token.len() > 1 {
            println!("RegexSplit: {:?}", split_token);
            let token = split_token.get(1).unwrap().to_string();
            let json = &self.arguments.clone().unwrap()[&token];
            value = match json {
                Value::String(value) => value.to_string(),
                Value::Bool(value) => value.to_string(),
                Value::Number(value) => value.to_string(),
                _ => "not supported".to_string(),
            };
            value = format!("{}{}{}", split_token[0], value, split_token[2]);
            (Some(value), Some(token))
        } else {
            (Some(value), None)
        }
    }
}
