pub fn div(var: &str) -> String {
    let mut output = String::new();
    output.push_str("<div>");
    output.push_str(var);
    output.push_str("</div>");
    output
}
