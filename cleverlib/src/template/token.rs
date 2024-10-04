pub mod token {
    use crate::template::token_type::TokenType;

    pub struct TemplateToken {
        pub raw: String,
        pub token_type: TokenType,
    }

    impl TemplateToken {
        fn create(&self, raw_string: String) -> TemplateToken {
            TemplateToken {
                raw: raw_string,
                token_type: TokenType::Name,
            }
        }
    }

    impl Token for TemplateToken {
        fn render(&self) -> String {
            todo!()
        }

        fn get_type(self) -> TokenType {
            self.token_type
        }

        fn get_match(&self) -> bool {
            todo!()
        }

        fn render_token(&self) -> bool {
            todo!()
        }

        fn raw(self) -> String {
            self.raw
        }
    }

    pub trait Token {
        fn render(&self) -> String;
        fn get_type(self) -> TokenType;
        fn get_match(&self) -> bool;
        fn render_token(&self) -> bool;
        fn raw(self) -> String;
    }
}
