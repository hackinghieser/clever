#[derive(Debug)]
pub enum TokenType {
    Text,
    Name,
    Hole,
    Format,
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Text => String::from("Text"),
            TokenType::Name => String::from("Name"),
            TokenType::Hole => String::from("Hole"),
            TokenType::Format => String::from("Format"),
        }
    }
}
