use std::collections::HashMap;

use htmlc::token::{generate_r, parse_html};

#[test]
fn tokenizes_plain_html() {
    let tokens = parse_html("<div><span>{name}</span></div>");
    let mut variable = HashMap::new();
    variable.insert("name", "Phat");
    let code = generate_r(tokens, "test" , variable);
    println!("{}", code);
}
