use life::templates::{DynamicView, render_dynamic};

#[test]
fn generated_dynamic_template_compiles_and_renders_raw_and_escaped_values() {
    let mut html = String::new();
    let view = DynamicView {
        value: r#"<script>alert("x")</script>"#,
        trusted_html: "<strong>trusted</strong>",
    };

    render_dynamic(&mut html, view);

    assert_eq!(
        html,
        concat!(
            "<p>&lt;script&gt;alert(&quot;x&quot;)&lt;/script&gt;</p>\n",
            "<section><strong>trusted</strong></section>\n",
        )
    );
}
