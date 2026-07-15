use std::collections::HashSet;

use crate::{error::TemplateError, util::is_valid_rust_variable_name};

#[derive(Debug)]
pub enum Token {
    Literal(String),
    Variable(String),
}

type VarCount = u32;

fn parse_html(html: &str) -> Result<(Vec<Token>, VarCount), TemplateError> {
    let mut tokens = vec![];
    let mut literal_index = 0;
    let mut variable_index = 0;
    let mut variable_count = 0;
    let mut current_variable = HashSet::new();
    let mut open_variable = false;
    for (index, char) in html.char_indices() {
        if char == '{' {
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
        if char == '}' {
            if !open_variable {
                return Err(TemplateError::MissingOpenVariable);
            }
            let variable = html[variable_index..index].trim();
            if variable.is_empty() {
                return Err(TemplateError::EmptyVariable);
            }
            if !is_valid_rust_variable_name(variable) {
                return Err(TemplateError::InvalidVariable);
            }
            tokens.push(Token::Variable(variable.to_string()));
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
    let mut current_var = 0;
    let mut use_module = String::new();
    let mut view_struct = String::new();
    let mut function = String::new();
    if variable_count > 0 {
        use_module.push_str("use crate::util::escape_html;");
        view_struct.push_str(&format!("pub struct {}View<'a> {{", struct_name));
        for num in 0..variable_count {
            view_struct.push_str(&format!("pub <var{}>: &'a str,", num));
        }
        view_struct.push('}');
        function.push_str(&format!(
            "pub fn render_{}(out: &mut String, view: {}View, escape: bool) {{",
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
            Token::Variable(variable) => {
                view_struct = view_struct.replace(&format!("<var{}>", current_var), &variable);
                function.push_str(&format!(
                    "if escape {{out.push_str(escape_html(view.{}));}} else {{out.push_str(view.{});}}",
                    &variable, &variable
                ));
                current_var += 1;
            }
        }
    }
    function.push('}');
    format!("{}{}{}", use_module, view_struct, function)
}

pub fn generate_code(
    html: &str,
    fn_name: &str,
    struct_name: &str,
) -> Result<String, TemplateError> {
    let (token, var_count) = parse_html(html)?;
    Ok(generate_r(token, fn_name, struct_name, var_count))
}
