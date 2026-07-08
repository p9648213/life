use crate::token::Token::{Literal, Variable};

#[derive(Debug)]
pub enum Token {
    Literal(String),
    Variable(String),
}

pub fn parse_html(html: &str) -> Vec<Token> {
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
    tokens
}

// pub fn render_home(ctx: &HomeTemplate<'_>, out: &mut String) {
//     out.push_str("<h1>");
//     crate::html::escape::text(out, ctx.title);
//     out.push_str("</h1><p>");
//     crate::html::escape::text(out, ctx.message);
//     out.push_str("</p>");
// }

pub fn generate_r(tokens: Vec<Token>, file_name: &str) -> String {
    let mut code = String::new();

    code.push_str(&format!("pub fn render_{}(out: &mut String) {{", {file_name}));
    
    for token in tokens {
        match token {
            Literal(literal) => code.push_str(&format!("out.push_str(\"{}\");", literal)),
            Variable(variable) => todo!(),
        }
    }

    code.push_str("}");

    code
}
