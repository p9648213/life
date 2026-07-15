use htmlc::{compiler::generate_code, error::TemplateError};

fn normalize_generated_rust(value: &str) -> String {
    let mut normalized = String::new();
    let mut in_string = false;
    let mut escaped = false;

    for character in value.chars() {
        if in_string {
            normalized.push(character);

            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
        } else if character == '"' {
            in_string = true;
            normalized.push(character);
        } else if !character.is_whitespace() {
            normalized.push(character);
        }
    }

    normalized
}

#[test]
fn renders_a_single_variable_between_literal_fragments() {
    let code = generate_code("<div><span>{name}</span></div>", "test", "Test")
        .expect("valid template should compile");
    let expected = r#"
        use crate::util::escape_html;
        pub struct TestView<'a> {
            pub name: &'a str,
        } 
        
        pub fn render_test(out: &mut String, view: TestView, escape: bool) {
            out.push_str("<div><span>");
            if escape {out.push_str(escape_html(view.name));} else {out.push_str(view.name);}
            out.push_str("</span></div>");
        }
        "#;
    assert_eq!(
        normalize_generated_rust(&code),
        normalize_generated_rust(expected)
    );
}

#[test]
fn renders_a_variable_after_multiple_static_elements() {
    let code = generate_code("<div></div><span>{name}</span>", "test", "Test")
        .expect("valid template should compile");
    let expected = r#"
        use crate::util::escape_html;
        pub struct TestView<'a> {
            pub name: &'a str,
        } 
        
        pub fn render_test(out: &mut String, view: TestView, escape: bool) {
            out.push_str("<div></div><span>");
            if escape {out.push_str(escape_html(view.name));} else {out.push_str(view.name);}
            out.push_str("</span>");
        }
        "#;

    assert_eq!(
        normalize_generated_rust(&code),
        normalize_generated_rust(expected)
    );
}

#[test]
fn renders_multiple_variables_separated_by_literals() {
    let code = generate_code("<div>{age}</div><span>{name}</span>", "test", "Test")
        .expect("valid template should compile");
    let expected = r#"
        use crate::util::escape_html;
        pub struct TestView<'a> {
            pub age: &'a str,
            pub name: &'a str,
        }
        
        pub fn render_test(out: &mut String, view: TestView, escape: bool) {
            out.push_str("<div>");
            if escape {out.push_str(escape_html(view.age));} else {out.push_str(view.age);}
            out.push_str("</div><span>");
            if escape {out.push_str(escape_html(view.name));} else {out.push_str(view.name);}
            out.push_str("</span>");
        }
        "#;

    assert_eq!(
        normalize_generated_rust(&code),
        normalize_generated_rust(expected)
    );
}

#[test]
fn renders_adjacent_variables_in_their_original_order() {
    let code = generate_code("<div>{age}{name}</div>", "test", "Test")
        .expect("valid template should compile");
    let expected = r#"
        use crate::util::escape_html;
        pub struct TestView<'a> {
            pub age: &'a str,
            pub name: &'a str,
        }
        
        pub fn render_test(out: &mut String, view: TestView, escape: bool) {
            out.push_str("<div>");
            if escape {out.push_str(escape_html(view.age));} else {out.push_str(view.age);}
            if escape {out.push_str(escape_html(view.name));} else {out.push_str(view.name);}
            out.push_str("</div>");
        }
        "#;

    assert_eq!(
        normalize_generated_rust(&code),
        normalize_generated_rust(expected)
    );
}

#[test]
fn escapes_quotes_in_html_attributes() {
    let code = generate_code(
        r#"<div class="container">{age}{name}</div>"#,
        "test",
        "Test",
    )
    .expect("valid template should compile");
    let expected = r#"
        use crate::util::escape_html;
        pub struct TestView<'a> {
            pub age: &'a str,
            pub name: &'a str,
        }
        
        pub fn render_test(out: &mut String, view: TestView, escape: bool) {
            out.push_str("<div class=\"container\">");
            if escape {out.push_str(escape_html(view.age));} else {out.push_str(view.age);}
            if escape {out.push_str(escape_html(view.name));} else {out.push_str(view.name);}
            out.push_str("</div>");
        }
        "#;
    assert_eq!(
        normalize_generated_rust(&code),
        normalize_generated_rust(expected)
    );
}

#[test]
fn escapes_quotes_backslashes_and_newlines_in_literal_html() {
    let code = generate_code("<a href=\"C:\\\\docs\">{label}</a>\n", "link", "Link")
        .expect("valid template should compile");

    let expected = r#"
        use crate::util::escape_html;
        pub struct LinkView<'a> {
            pub label: &'a str,
        }

        pub fn render_link(out: &mut String, view: LinkView, escape: bool) {
            out.push_str("<a href=\"C:\\\\docs\">");
            if escape {out.push_str(escape_html(view.label));} else {out.push_str(view.label);}
            out.push_str("</a>\n");
        }
        "#;

    assert_eq!(
        normalize_generated_rust(&code),
        normalize_generated_rust(expected)
    );
}

#[test]
fn static_template_has_no_view_struct_or_parameter() {
    let code = generate_code("<!doctype html><p>Welcome</p>", "welcome", "Welcome")
        .expect("valid template should compile");
    let expected = r#"
        pub fn render_welcome(out: &mut String) {
            out.push_str("<!doctype html><p>Welcome</p>");
        }
        "#;

    assert_eq!(
        normalize_generated_rust(&code),
        normalize_generated_rust(expected)
    );
}

#[test]
fn preserves_unicode_html_around_a_variable() {
    let code = generate_code("<p>Chào, {name} 👋</p>", "greeting", "Greeting")
        .expect("valid template should compile");
    let expected = r#"
        use crate::util::escape_html;
        pub struct GreetingView<'a> {
            pub name: &'a str,
        }

        pub fn render_greeting(out: &mut String, view: GreetingView, escape: bool) {
            out.push_str("<p>Chào, ");
            if escape {out.push_str(escape_html(view.name));} else {out.push_str(view.name);}
            out.push_str(" 👋</p>");
        }
        "#;

    assert_eq!(
        normalize_generated_rust(&code),
        normalize_generated_rust(expected)
    );
}

#[test]
fn preserves_meaningful_whitespace_in_literal_html() {
    let code = generate_code("<p>Hello world</p>", "message", "Message")
        .expect("valid template should compile");

    assert!(
        code.contains(r#"out.push_str("<p>Hello world</p>");"#),
        "generated code did not preserve the literal space: {code}"
    );
}

#[test]
fn conditionally_escapes_runtime_values() {
    let code = generate_code("<p>{value}</p>", "message", "Message")
        .expect("valid template should compile");

    assert!(
        code.contains("if escape {out.push_str(escape_html(view.value));} else {out.push_str(view.value);}"),
        "generated renderer must escape runtime values when requested: {code}"
    );
    assert_eq!(
        code.matches("escape_html(view.value)").count(),
        1,
        "the generated renderer should pass the runtime value through escaping once: {code}"
    );
}

#[test]
fn repeated_variable_uses_one_context_field() {
    let code = generate_code("<h1>{title}</h1><p>{title}</p>", "article", "Article")
        .expect("valid template should compile");

    assert_eq!(
        code.matches("title: &'a str").count(),
        1,
        "a repeated variable should create only one context field: {code}"
    );
    assert_eq!(
        code.matches("view.title").count(),
        4,
        "each variable occurrence should read the shared context field in both escape branches: {code}"
    );
}

#[test]
fn rejects_an_empty_variable() {
    let result = generate_code("<p>{}</p>", "message", "Message");

    assert!(matches!(result, Err(TemplateError::EmptyVariable)));
}

#[test]
fn rejects_an_unclosed_variable() {
    let result = generate_code("<p>{title</p>", "message", "Message");

    assert!(result.is_err(), "an opening brace must have a closing brace");
}

#[test]
fn rejects_a_stray_closing_brace() {
    let result = generate_code("<p>title}</p>", "message", "Message");

    assert!(result.is_err(), "a closing brace must follow an opening brace");
}

#[test]
fn rejects_an_invalid_variable_name() {
    let result = generate_code("<p>{display name}</p>", "message", "Message");

    assert!(
        result.is_err(),
        "variable names must be valid Rust field identifiers"
    );
}
