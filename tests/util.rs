use life::util::escape_html;

#[test]
fn escape_html_appends_all_text_escapes_to_the_existing_buffer() {
    let mut output = String::from("prefix:");

    escape_html("&<>\"'", &mut output);

    assert_eq!(output, "prefix:&amp;&lt;&gt;&quot;&#x27;");
}
