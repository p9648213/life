use htmlc::compiler::{generate_r, parse_html};

fn remove_whitespace(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>()
}

#[test]
fn html_to_rust_code() {
    let (tokens, var_count) = parse_html("<div><span>{name}</span></div>");
    let code = generate_r(tokens, "test", "Test", var_count);
    let expected = r#"
        pub struct TestView<'a> {
            name: &'a str,
        } 
        
        pub fn render_test(out: &mut String, view: TestView) {
            out.push_str("<div><span>");
            out.push_str(view.name);
            out.push_str("</span></div>");
        }
        "#;

    assert_eq!(remove_whitespace(&code), remove_whitespace(expected));
}

#[test]
fn html_to_rust_code_2() {
    let (tokens, var_count) = parse_html("<div></div><span>{name}</span>");
    let code = generate_r(tokens, "test", "Test", var_count);
    let expected = r#"
        pub struct TestView<'a> {
            name: &'a str,
        } 
        
        pub fn render_test(out: &mut String, view: TestView) {
            out.push_str("<div></div><span>");
            out.push_str(view.name);
            out.push_str("</span>");
        }
        "#;

    assert_eq!(remove_whitespace(&code), remove_whitespace(expected));
}

#[test]
fn html_to_rust_code_3() {
    let (tokens, var_count) = parse_html("<div>{age}</div><span>{name}</span>");
    let code = generate_r(tokens, "test", "Test", var_count);
    let expected = r#"
        pub struct TestView<'a> {
            age: &'a str,
            name: &'a str,
        }
        
        pub fn render_test(out: &mut String, view: TestView) {
            out.push_str("<div>");
            out.push_str(view.age);
            out.push_str("</div><span>");
            out.push_str(view.name);
            out.push_str("</span>");
        }
        "#;

    assert_eq!(remove_whitespace(&code), remove_whitespace(expected));
}

#[test]
fn html_to_rust_code_4() {
    let (tokens, var_count) = parse_html("<div>{age}{name}</div>");
    let code = generate_r(tokens, "test", "Test", var_count);
    let expected = r#"
        pub struct TestView<'a> {
            age: &'a str,
            name: &'a str,
        }
        
        pub fn render_test(out: &mut String, view: TestView) {
            out.push_str("<div>");
            out.push_str(view.age);
            out.push_str(view.name);
            out.push_str("</div>");
        }
        "#;

    assert_eq!(remove_whitespace(&code), remove_whitespace(expected));
}
