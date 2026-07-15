use std::collections::HashSet;

use crate::{error::TemplateError, util::is_valid_rust_variable_name};

type VarCount = u32;
type Escape = bool;

#[derive(Debug)]
pub enum Token {
    Literal(String),
    Variable(String, Escape),
}

fn parse_html(html: &str) -> Result<(Vec<Token>, VarCount), TemplateError> {
    let mut tokens = vec![];
    let mut literal_index = 0;
    let mut variable_index = 0;
    let mut variable_count = 0;
    let mut current_variable = HashSet::new();
    let mut open_variable = false;
    for (index, ch) in html.char_indices() {
        if ch == '{' {
            if open_variable {
                return Err(TemplateError::UnCloseVariable);
            }
            open_variable = true;
            let literal = html[literal_index..index].to_string();
            if !literal.is_empty() {
                tokens.push(Token::Literal(literal));
            }
            variable_index = index + 1;
        }
        if ch == '}' {
            if !open_variable {
                return Err(TemplateError::MissingOpenVariable);
            }
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
            if !current_variable.contains(variable) {
                current_variable.insert(variable);
                variable_count += 1;
            }
            open_variable = false;
        }
    }
    if open_variable {
        return Err(TemplateError::UnCloseVariable);
    }
    tokens.push(Token::Literal(html[literal_index..].to_string()));
    Ok((tokens, variable_count))
}

fn generate_r(tokens: Vec<Token>, fn_name: &str, struct_name: &str, variable_count: u32) -> String {
    let mut var_pos = 0;
    let mut current_var = HashSet::new();
    let mut view_struct = String::new();
    let mut function = String::new();
    if variable_count > 0 {
        view_struct.push_str(&format!("pub struct {}View<'a> {{", struct_name));
        for num in 0..variable_count {
            view_struct.push_str(&format!("pub <var{}>: &'a str,", num));
        }
        view_struct.push('}');
        function.push_str(&format!(
            "pub fn render_{}(out: &mut String, view: {}View) {{",
            fn_name.to_ascii_lowercase(),
            struct_name
        ));
    } else {
        function.push_str(&format!(
            "pub fn render_{}(out: &mut String) {{",
            fn_name.to_ascii_lowercase(),
        ));
    }
    for token in tokens {
        match token {
            Token::Literal(literal) => {
                let literal = format!("{:?}", literal);
                function.push_str(&format!(r#"out.push_str({});"#, literal))
            }
            Token::Variable(variable, escape) => {
                if !current_var.contains(&variable) {
                    view_struct = view_struct.replace(&format!("<var{}>", var_pos), &variable);
                    var_pos += 1;
                }
                if escape {
                    function.push_str(&format!(
                        "crate::util::escape_html(view.{}, out);",
                        variable
                    ));
                } else {
                    function.push_str(&format!("out.push_str(view.{});", variable));
                }
                current_var.insert(variable);
            }
        }
    }
    function.push('}');
    format!("{}{}", view_struct, function)
}

pub fn generate_code(
    html: &str,
    fn_name: &str,
    struct_name: &str,
) -> Result<String, TemplateError> {
    let (token, var_count) = parse_html(html)?;
    Ok(generate_r(token, fn_name, struct_name, var_count))
}
