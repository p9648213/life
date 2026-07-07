pub fn render(tag: &str, child: &str, sibling_a: &str, sibling_b: &str) -> String {
    let mut output = String::new();
    output.push_str(sibling_a);
    output.push_str(&format!("<{tag}>"));
    output.push_str(child);
    output.push_str(&format!("</{tag}>"));
    output.push_str(sibling_b);
    output
}
