use life::templates::{AdminCardIndexView, render_admin_card_index};

#[test]
fn generated_template_escapes_only_marked_occurrences() {
    let mut html = String::new();
    let view = AdminCardIndexView {
        title: r#"<script>alert("x")</script>"#,
        body: "<em>trusted</em>",
    };

    render_admin_card_index(&mut html, view);

    assert_eq!(
        html,
        concat!(
            "<!doctype html>\n",
            "<main>\n",
            "  <h1>",
            "&lt;script&gt;alert(&quot;x&quot;)&lt;/script&gt;",
            "<script>alert(\"x\")</script>",
            "<script>alert(\"x\")</script>",
            "<em>trusted</em>",
            "</h1>\n",
            "</main>\n",
        )
    );
}
