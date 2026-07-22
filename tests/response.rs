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
fn new_response_serializes_without_custom_headers() {
    let bytes = Response::new(StatusCode::Ok, b"abc".to_vec()).to_bytes();
    let response = String::from_utf8(bytes).unwrap();

    assert_eq!(
        response,
        "HTTP/1.1 200 OK\r\n\
Content-Length: 3\r\n\
Connection: close\r\n\
\r\n\
abc"
    );
}

#[test]
fn text_plain_response_adds_its_trusted_content_type() {
    let bytes = Response::text_plain(StatusCode::Ok, "hello").to_bytes();
    let response = String::from_utf8(bytes).unwrap();

    assert!(response.contains("Content-Type: text/plain\r\n"));
    assert!(response.ends_with("\r\n\r\nhello"));
}

#[test]
fn valid_custom_header_is_serialized() {
    let response = Response::new(StatusCode::Ok, b"abc".to_vec());
    response
        .add_header("X-Test", "yes")
        .expect("valid response header should be accepted");

    let serialized = String::from_utf8(response.to_bytes()).unwrap();

    assert!(serialized.contains("X-Test: yes\r\n"));
    assert!(serialized.contains("Content-Length: 3\r\n"));
    assert_eq!(serialized.matches("Content-Length:").count(), 1);
}

#[test]
fn add_header_rejects_cr_or_lf_in_header_name() {
    for name in ["X-Test\rInjected", "X-Test\nInjected"] {
        let response = Response::new(StatusCode::Ok, b"ok".to_vec());
        let result = response.add_header(name, "safe");

        assert!(
            result.is_err(),
            "invalid header name should be rejected: {name:?}"
        );
    }
}

#[test]
fn add_header_rejects_cr_or_lf_in_header_value() {
    for value in [
        "safe\rInjected: yes",
        "safe\nInjected: yes",
        "safe\r\nInjected: yes",
    ] {
        let response = Response::new(StatusCode::Ok, b"ok".to_vec());
        let result = response.add_header("X-Test", value);

        assert!(
            result.is_err(),
            "invalid header value should be rejected: {value:?}"
        );
    }
}

#[test]
fn add_header_rejects_serializer_owned_framing_headers_case_insensitively() {
    for name in [
        "Content-Length",
        "content-length",
        "cOnTeNt-LeNgTh",
        "Transfer-Encoding",
        "transfer-encoding",
        "tRaNsFeR-EnCoDiNg",
        "Connection",
        "connection",
        "cOnNeCtIoN",
    ] {
        let response = Response::new(StatusCode::Ok, b"ok".to_vec());
        let result = response.add_header(name, "invalid");

        assert!(
            result.is_err(),
            "serializer-owned header should be rejected: {name}"
        );
    }
}

#[test]
fn rejected_header_is_not_serialized() {
    let response = Response::new(StatusCode::Ok, b"ok".to_vec());

    assert!(
        response
            .add_header("X-Test", "safe\r\nInjected: yes")
            .is_err()
    );

    let serialized = String::from_utf8(response.to_bytes()).unwrap();

    assert!(!serialized.contains("X-Test:"));
    assert!(!serialized.contains("Injected:"));
    assert_eq!(serialized.matches("Content-Length:").count(), 1);
}
