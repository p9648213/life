use htmlc::token::Tokenizer;

#[test]
fn tokenizes_plain_html() {
    let tokenizer = Tokenizer::parse_html("<div><span>{name}</span></div>");
}
