use life::http::response::{Response, StatusCode};

#[test]
fn html_response_serializes_status_headers_blank_line_and_body() {
    let bytes = Response::html(StatusCode::Ok, "<h1>Hello</h1>").to_bytes();
    let response = String::from_utf8(bytes).unwrap();

    assert_eq!(
        response,
        "HTTP/1.1 200 OK\r\n\
Content-Type: text/html; charset=utf-8\r\n\
Content-Length: 14\r\n\
Connection: close\r\n\
\r\n\
<h1>Hello</h1>"
    );
}

#[test]
fn html_response_can_serialize_404() {
    let bytes = Response::html(StatusCode::NotFound, "<h1>Not Found</h1>").to_bytes();
    let response = String::from_utf8(bytes).unwrap();

    assert!(response.starts_with("HTTP/1.1 404 Not Found\r\n"));
}

#[test]
fn content_length_counts_body_bytes_not_characters() {
    let bytes = Response::html(StatusCode::Ok, "h\u{00e9}").to_bytes();
    let response = String::from_utf8(bytes).unwrap();

    assert!(response.contains("Content-Length: 3\r\n"));
}

#[test]
fn caller_supplied_content_length_is_ignored() {
    let bytes = Response::new(
        StatusCode::Ok,
        vec![("Content-Length", "999"), ("X-Test", "yes")],
        b"abc".to_vec(),
    )
    .to_bytes();
    let response = String::from_utf8(bytes).unwrap();

    assert!(response.contains("X-Test: yes\r\n"));
    assert!(response.contains("Content-Length: 3\r\n"));
    assert!(!response.contains("Content-Length: 999\r\n"));
    assert_eq!(response.matches("Content-Length:").count(), 1);
}
