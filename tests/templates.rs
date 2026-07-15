use life::templates::{
    AdminCardIndexView, AdminCardPage1View, AdminCardPage2View, AdminCardPage3View,
    render_admin_card_index, render_admin_card_page_3, render_admin_card_page1,
    render_admin_card_page2,
};

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

#[test]
fn multiple_dynamic_filename_templates_compile_and_render_together() {
    let mut html = String::new();

    render_admin_card_page2(
        &mut html,
        AdminCardPage2View {
            value: "<page-2>",
        },
    );
    render_admin_card_page1(
        &mut html,
        AdminCardPage1View {
            value: "<page1 & value>",
        },
    );
    render_admin_card_page_3(
        &mut html,
        AdminCardPage3View {
            value: "<page_3>",
        },
    );

    assert_eq!(
        html,
        concat!(
            "<p>&lt;page-2&gt;</p>\n",
            "<p>&lt;page1 &amp; value&gt;</p>\n",
            "<p>&lt;page_3&gt;</p>\n",
        )
    );
}
