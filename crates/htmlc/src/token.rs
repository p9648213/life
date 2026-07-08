use crate::token::Token::{Literal, Variable};

#[derive(Debug)]
enum Token {
    Literal(String),
    Variable(String),
}

#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn parse_html(html: &str) -> Self {
        let mut tokens = vec![];
        let mut literal_index = 0;
        let mut variable_index = 0;

        for (index, char) in html.char_indices() {
            if char == '{' {
                tokens.push(Literal(html[literal_index..index].to_string()));
                variable_index = index + 1;
            }

            if char == '}' {
                tokens.push(Variable(html[variable_index..index].to_string()));
                literal_index = index + 1;
            }
        }

        tokens.push(Literal(html[literal_index..].to_string()));

        Self { tokens }
    }
}
