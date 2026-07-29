use crate::{error::TemplateError, util::is_valid_rust_variable_name};
use std::collections::HashSet;

type Escape = bool;

#[derive(Debug)]
pub enum Token {
    Literal(String),
    Variable(String, Escape),
}

fn parse_html(html: &str) -> Result<Vec<Token>, TemplateError> {
    let mut tokens = vec![];
    let mut index = 0;
    let mut literal_index = 0;
    let mut variable_index = 0;
    let mut open_variable = false;
    while index < html.len() {
        if html[index..].starts_with("{@") {
            if open_variable {
                return Err(TemplateError::UnCloseVariable);
            }
            open_variable = true;
            let literal = html[literal_index..index].to_string();
            if !literal.is_empty() {
                tokens.push(Token::Literal(literal));
            }
            variable_index = index + 2;
            index += 2;
            continue;
        }
        let ch = html[index..].chars().next().unwrap();
        if ch == '}' && open_variable {
            let mut variable = html[variable_index..index].trim();
            if variable.is_empty() {
                return Err(TemplateError::EmptyVariable);
            }
            if !is_valid_rust_variable_name(variable) {
                return Err(TemplateError::InvalidVariable);
            }
            if variable.contains(":") {
                let mut var_part = variable.split(":");
                variable = var_part.next().unwrap();
                let mut escape = false;
                for operation in var_part {
                    match operation {
                        "escape" => escape = true,
                        _ => return Err(TemplateError::InvalidOperation),
                    }
                }
                tokens.push(Token::Variable(variable.to_string(), escape));
            } else {
                tokens.push(Token::Variable(variable.to_string(), false));
            }
            literal_index = index + 1;
            open_variable = false;
        }
        index += ch.len_utf8();
    }
    if open_variable {
        return Err(TemplateError::UnCloseVariable);
    }
    tokens.push(Token::Literal(html[literal_index..].to_string()));
    Ok(tokens)
}

fn generate_r(tokens: &[Token], fn_name: &str, struct_name: &str) -> String {
    let mut seen_variables = HashSet::new();
    let mut variables = Vec::new();
    let mut view_struct = String::new();
    let mut function_header = String::new();
    let mut function_body = String::new();
    for token in tokens {
        match token {
            Token::Literal(literal) => {
                let literal = format!("{:?}", literal);
                function_body.push_str(&format!(r#"out.push_str({});"#, literal))
            }
            Token::Variable(name, escape) => {
                if *escape {
                    function_body
                        .push_str(&format!("crate::util::escape_html(view.{}, out);", name));
                } else {
                    function_body.push_str(&format!("out.push_str(view.{});", name));
                }
                if seen_variables.insert(name.as_str()) {
                    variables.push(name.as_str());
                }
            }
        }
    }
    if !variables.is_empty() {
        function_header.push_str(&format!(
            "pub fn render_{}(out: &mut String, view: {}View) {{",
            fn_name.to_ascii_lowercase(),
            struct_name
        ));
        view_struct.push_str(&format!("pub struct {}View<'a> {{", struct_name));
        for var in &variables {
            view_struct.push_str(&format!("pub {}: &'a str,", var));
        }
        view_struct.push('}');
    } else {
        function_header.push_str(&format!(
            "pub fn render_{}(out: &mut String) {{",
            fn_name.to_ascii_lowercase(),
        ));
    }
    let total_length = view_struct.len() + function_header.len() + function_body.len() + 1;
    let mut generated = String::with_capacity(total_length);
    generated.push_str(&view_struct);
    generated.push_str(&function_header);
    generated.push_str(&function_body);
    generated.push('}');
    generated
}

pub fn generate_code(
    html: &str,
    fn_name: &str,
    struct_name: &str,
) -> Result<String, TemplateError> {
    let token = parse_html(html)?;
    Ok(generate_r(&token, fn_name, struct_name))
}
