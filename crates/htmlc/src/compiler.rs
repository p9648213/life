#[derive(Debug)]
pub enum Token {
    Literal(String),
    Variable(String),
}

type VarCount = u32;

fn parse_html(html: &str) -> (Vec<Token>, VarCount) {
    let mut tokens = vec![];
    let mut literal_index = 0;
    let mut variable_index = 0;
    let mut variable_count = 0;
    for (index, char) in html.char_indices() {
        if char == '{' {
            let literal = html[literal_index..index].to_string();
            if !literal.is_empty() {
                tokens.push(Token::Literal(literal));
            }
            variable_index = index + 1;
        }
        if char == '}' {
            tokens.push(Token::Variable(html[variable_index..index].to_string()));
            literal_index = index + 1;
            variable_count += 1;
        }
    }
    tokens.push(Token::Literal(html[literal_index..].to_string()));
    (tokens, variable_count)
}

fn generate_r(tokens: Vec<Token>, fn_name: &str, struct_name: &str, variable_count: u32) -> String {
    let mut current_var = 0;
    let mut view_struct = String::new();
    let mut function = String::new();
    if variable_count > 0 {
        view_struct.push_str(&format!("pub struct {}View<'a> {{", struct_name));
        for num in 0..variable_count {
            view_struct.push_str(&format!("<var{}>: &'a str,", num));
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
            Token::Variable(variable) => {
                view_struct = view_struct.replace(&format!("<var{}>", current_var), &variable);
                function.push_str(&format!("out.push_str(view.{});", variable));
                current_var += 1;
            }
        }
    }
    function.push('}');
    format!("{}{}", view_struct, function)
}

pub fn generate_code(html: &str, fn_name: &str, struct_name: &str) -> String {
    let (token, var_count) = parse_html(html);
    generate_r(token, fn_name, struct_name, var_count)
}
