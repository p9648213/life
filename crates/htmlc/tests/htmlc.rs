use htmlc::compiler::generate_code;

fn remove_whitespace(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>()
}

#[test]
fn renders_a_single_variable_between_literal_fragments() {
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
fn renders_a_variable_after_multiple_static_elements() {
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
fn renders_multiple_variables_separated_by_literals() {
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
fn renders_adjacent_variables_in_their_original_order() {
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

#[test]
fn escapes_quotes_in_html_attributes() {
    let code = generate_code(
        r#"<div class="container">{age}{name}</div>"#,
        "test",
        "Test",
    );
    let expected = r#"
        pub struct TestView<'a> {
            age: &'a str,
            name: &'a str,
        }
        
        pub fn render_test(out: &mut String, view: TestView) {
            out.push_str("<div class=\"container\">");
            out.push_str(view.age);
            out.push_str(view.name);
            out.push_str("</div>");
        }
        "#;
    assert_eq!(remove_whitespace(&code), remove_whitespace(expected));
}

#[test]
fn escapes_quotes_backslashes_and_newlines_in_literal_html() {
    let code = generate_code(
        "<a href=\"C:\\\\docs\">{label}</a>\n",
        "link",
        "Link",
    );

    let expected = r#"
        pub struct LinkView<'a> {
            label: &'a str,
        }

        pub fn render_link(out: &mut String, view: LinkView) {
            out.push_str("<a href=\"C:\\\\docs\">");
            out.push_str(view.label);
            out.push_str("</a>\n");
        }
        "#;

    assert_eq!(remove_whitespace(&code), remove_whitespace(expected));
}

#[test]
fn static_template_has_no_view_lifetime_or_fields() {
    let code = generate_code("<!doctype html><p>Welcome</p>", "welcome", "Welcome");
    let expected = r#"
        pub struct WelcomeView {}

        pub fn render_welcome(out: &mut String, view: WelcomeView) {
            out.push_str("<!doctype html><p>Welcome</p>");
        }
        "#;

    assert_eq!(remove_whitespace(&code), remove_whitespace(expected));
}

#[test]
fn preserves_unicode_html_around_a_variable() {
    let code = generate_code("<p>Chào, {name} 👋</p>", "greeting", "Greeting");
    let expected = r#"
        pub struct GreetingView<'a> {
            name: &'a str,
        }

        pub fn render_greeting(out: &mut String, view: GreetingView) {
            out.push_str("<p>Chào, ");
            out.push_str(view.name);
            out.push_str(" 👋</p>");
        }
        "#;

    assert_eq!(remove_whitespace(&code), remove_whitespace(expected));
}
