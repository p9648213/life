use htmlc::token::Tokenizer;

#[test]
fn tokenizes_plain_html() {
    let tokenizer = Tokenizer::parse_html("<div>{name}<span>{age}</span></div>");
    println!("{:#?}", tokenizer);
}
