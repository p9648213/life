use life::http::request::{HttpMethod, Request};

fn parse_ok(data: &[u8]) -> Request<'_> {
    Request::parse(data).expect("request should parse")
}

#[test]
fn parses_valid_get_request() {
    let request = parse_ok(b"GET /notes HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert!(matches!(request.method(), HttpMethod::Get));
    assert_eq!(request.target_path(), "/notes");
    assert_eq!(request.path(), "/notes");
    assert_eq!(request.version(), "HTTP/1.1");
    assert_eq!(request.get_header("host"), Some("localhost"));
    assert_eq!(request.get_header("Host"), Some("localhost"));
    assert_eq!(request.get_header("HOST"), Some("localhost"));
    assert_eq!(request.body(), b"");
    assert!(request.query().is_empty());
}

#[test]
fn parses_valid_post_body_using_content_length() {
    let request = parse_ok(
        b"POST /notes HTTP/1.1\r\nHost: localhost\r\nContent-Length: 22\r\n\r\ntitle=Hello&body=World",
    );

    assert!(matches!(request.method(), HttpMethod::Post));
    assert_eq!(request.target_path(), "/notes");
    assert_eq!(request.path(), "/notes");
    assert_eq!(request.body(), b"title=Hello&body=World");
}

#[test]
fn separates_path_from_query_parameters() {
    let request = parse_ok(b"GET /resources?id=123 HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert_eq!(request.target_path(), "/resources?id=123");
    assert_eq!(request.path(), "/resources");
    assert_eq!(request.query().get("id"), Some(&"123"));
}

#[test]
fn parses_empty_query_values() {
    let request = parse_ok(b"GET /resources?id= HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert_eq!(request.path(), "/resources");
    assert_eq!(request.query().get("id"), Some(&""));
}

#[test]
fn repeated_query_keys_keep_the_last_value() {
    let request =
        parse_ok(b"GET /resources?id=first&id=second HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert_eq!(request.path(), "/resources");
    assert_eq!(request.query().get("id"), Some(&"second"));
}

#[test]
fn encoded_and_plus_query_values_are_preserved_without_decoding() {
    let request =
        parse_ok(b"GET /search?q=hello%20world+again HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert_eq!(request.path(), "/search");
    assert_eq!(request.query().get("q"), Some(&"hello%20world+again"));
}

#[test]
fn rejects_query_pairs_without_equals() {
    let result = Request::parse(b"GET /resources?id HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert!(result.is_err());
}

#[test]
fn rejects_empty_query_keys() {
    let result = Request::parse(b"GET /resources?=123 HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert!(result.is_err());
}

#[test]
fn rejects_empty_query_string() {
    let result = Request::parse(b"GET /resources? HTTP/1.1\r\nHost: localhost\r\n\r\n");

    assert!(result.is_err());
}

#[test]
fn preserves_colons_inside_header_values() {
    let request = parse_ok(b"GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\n\r\n");

    assert_eq!(request.get_header("HOST"), Some("127.0.0.1:8080"));
}

#[test]
fn content_length_name_is_case_insensitive() {
    let request = parse_ok(b"POST / HTTP/1.1\r\ncontent-length: 3\r\n\r\nabc");

    assert_eq!(request.body(), b"abc");
}

#[test]
fn lowercase_content_length_is_used_to_detect_an_incomplete_body() {
    let result = Request::parse(b"POST / HTTP/1.1\r\ncontent-length: 5\r\n\r\nabc");

    assert!(result.is_err());
}

#[test]
fn rejects_input_without_header_body_separator() {
    let result = Request::parse(b"GET / HTTP/1.1\r\nHost: localhost\r\n");

    assert!(result.is_err());
}

#[test]
fn rejects_request_line_with_missing_part() {
    let result = Request::parse(b"GET /\r\nHost: localhost\r\n\r\n");

    assert!(result.is_err());
}

#[test]
fn rejects_request_line_with_extra_part() {
    let result = Request::parse(b"GET / HTTP/1.1 unexpected\r\nHost: localhost\r\n\r\n");

    assert!(result.is_err());
}

#[test]
fn rejects_unsupported_http_version() {
    let result = Request::parse(b"GET / HTTP/1.0\r\nHost: localhost\r\n\r\n");

    assert!(result.is_err());
}

#[test]
fn rejects_header_without_colon() {
    let result = Request::parse(b"GET / HTTP/1.1\r\nBroken-Header\r\n\r\n");

    assert!(result.is_err());
}

#[test]
fn rejects_invalid_content_length() {
    let result = Request::parse(b"POST / HTTP/1.1\r\nContent-Length: three\r\n\r\nabc");

    assert!(result.is_err());
}

#[test]
fn rejects_body_shorter_than_content_length() {
    let result = Request::parse(b"POST / HTTP/1.1\r\nContent-Length: 5\r\n\r\nabc");

    assert!(result.is_err());
}

#[test]
fn rejects_body_longer_than_content_length() {
    let result = Request::parse(b"POST / HTTP/1.1\r\nContent-Length: 3\r\n\r\nabcdef");

    assert!(result.is_err());
}

#[test]
fn rejects_post_body_without_content_length() {
    let result = Request::parse(b"POST / HTTP/1.1\r\nHost: localhost\r\n\r\nabc");

    assert!(result.is_err());
}

#[test]
fn rejects_get_body_without_content_length() {
    let result = Request::parse(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\nunexpected");

    assert!(result.is_err());
}

#[test]
fn very_short_input_returns_error_instead_of_panicking() {
    for data in [b"".as_slice(), b"G", b"GET", b"GET / HTTP/1.1\r"] {
        let result = Request::parse(data);
        assert!(result.is_err(), "input should be rejected: {data:?}");
    }
}

#[test]
fn rejects_non_utf8_request_line_or_headers() {
    let result = Request::parse(b"GET / HTTP/1.1\r\nX-Binary: \xff\r\n\r\n");

    assert!(result.is_err());
}
