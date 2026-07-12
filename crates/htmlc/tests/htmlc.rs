use htmlc::compiler::generate_code;

fn remove_whitespace(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>()
}

#[test]
fn html_to_rust_code() {
    let code = generate_code("<div><span>{name}</span></div>", "test", "Test");
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
    let code = generate_code("<div></div><span>{name}</span>", "test", "Test");
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
    let code = generate_code("<div>{age}</div><span>{name}</span>", "test", "Test");
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
    let code = generate_code("<div>{age}{name}</div>", "test", "Test");
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
