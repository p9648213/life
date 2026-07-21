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
fn valid_custom_header_is_serialized() {
    let bytes = Response::new(StatusCode::Ok, vec![("X-Test", "yes")], b"abc".to_vec())
        .expect("valid response headers should be accepted")
        .to_bytes();
    let response = String::from_utf8(bytes).unwrap();

    assert!(response.contains("X-Test: yes\r\n"));
    assert!(response.contains("Content-Length: 3\r\n"));
    assert_eq!(response.matches("Content-Length:").count(), 1);
}

#[test]
fn response_new_rejects_cr_or_lf_in_header_name() {
    for name in ["X-Test\rInjected", "X-Test\nInjected"] {
        let result = Response::new(StatusCode::Ok, vec![(name, "safe")], b"ok".to_vec());

        assert!(
            result.is_err(),
            "invalid header name should be rejected: {name:?}"
        );
    }
}

#[test]
fn response_new_rejects_cr_or_lf_in_header_value() {
    for value in [
        "safe\rInjected: yes",
        "safe\nInjected: yes",
        "safe\r\nInjected: yes",
    ] {
        let result = Response::new(StatusCode::Ok, vec![("X-Test", value)], b"ok".to_vec());

        assert!(
            result.is_err(),
            "invalid header value should be rejected: {value:?}"
        );
    }
}

#[test]
fn response_new_rejects_serializer_owned_framing_headers_case_insensitively() {
    for name in [
        "Content-Length",
        "content-length",
        "Transfer-Encoding",
        "transfer-encoding",
        "Connection",
        "connection",
    ] {
        let result = Response::new(StatusCode::Ok, vec![(name, "invalid")], b"ok".to_vec());

        assert!(
            result.is_err(),
            "serializer-owned header should be rejected: {name}"
        );
    }
}
