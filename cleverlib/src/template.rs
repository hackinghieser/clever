use regex::Regex;
use token::token::TemplateToken;
use token_type::TokenType;
pub mod alignment;
pub mod format;
pub mod hole;
pub mod name;
pub mod text;
pub mod token;
pub mod token_type;

pub struct Template {
    pub tokens: Vec<TemplateToken>,
}

impl Template {
    pub fn build(&self, raw_event: String) {
        //try to find any hole matches
        let regex = Regex::new(r"/.*?(\{[a-zA-Z0-9_]+\}|\{[0-9]+\}).?/gm").unwrap();
        let regex_single = Regex::new(r"/.*?(\{[a-zA-Z0-9_]+\}|\{[0-9]+\}).?/gm").unwrap();
        let tokens: Vec<&str> = raw_event.split(" ").collect();
        let contains_holes = regex.find(raw_event.as_str());
        if let None = contains_holes {
            println!("No wholes found");
            return;
        }
        for raw_token in tokens.iter() {
            let is_hole = regex_single.find(raw_token);
            if let None = is_hole {
                println!("{} is not a hole", raw_token)
            }
            let temp = Template::parse_token(raw_token.to_string());
        }
    }

    fn parse_token(token: String) -> TemplateToken {
        let temp = TemplateToken {
            token_type: token_type::TokenType::Text,
            raw: token,
        };
        temp
    }

    fn get_hole_type(&self, token: String) -> TokenType {
        TokenType::Text
    }
}
