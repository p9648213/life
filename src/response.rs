pub struct Response<'a> {
    status_code: u16,
    headers: Vec<(&'a str, &'a str)>,
    body_bytes: Vec<u8>,
}

impl<'a> Response<'a> {
    fn new(status_code: u16, headers: Vec<(&'a str, &'a str)>, body_bytes: Vec<u8>) -> Self {
        Self {
            status_code,
            headers,
            body_bytes,
        }
    }

    fn reason_phrase(status_code: u16) -> &'static str {
        match status_code {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = Vec::new();

        response.extend_from_slice(
            format!(
                "HTTP/1.1 {} {}\r\n",
                self.status_code,
                Self::reason_phrase(self.status_code)
            )
            .as_bytes(),
        );

        for header in &self.headers {
            if !header.0.eq_ignore_ascii_case("Content-Length") {
                response.extend_from_slice(format!("{}: {}\r\n", header.0, header.1).as_bytes());
            }
        }
        response
            .extend_from_slice(format!("Content-Length: {}\r\n", self.body_bytes.len()).as_bytes());
        response.extend_from_slice(b"Connection: close\r\n");
        response.extend_from_slice(b"\r\n");
        response.extend_from_slice(&self.body_bytes);

        response
    }


    pub fn html(status_code: u16, text: &str) -> Self {
        Self::new(
            status_code,
            vec![(
                "Content-Type",
                "text/html; charset=utf-8",
            )],
            text.as_bytes().to_vec(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Response;

    #[test]
    fn html_response_serializes_status_headers_blank_line_and_body() {
        let bytes = Response::html(200, "<h1>Hello</h1>").to_bytes();
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
        let bytes = Response::html(404, "<h1>Not Found</h1>").to_bytes();
        let response = String::from_utf8(bytes).unwrap();

        assert!(response.starts_with("HTTP/1.1 404 Not Found\r\n"));
    }

    #[test]
    fn content_length_counts_body_bytes_not_characters() {
        let bytes = Response::html(200, "h\u{00e9}").to_bytes();
        let response = String::from_utf8(bytes).unwrap();

        assert!(response.contains("Content-Length: 3\r\n"));
    }

    #[test]
    fn caller_supplied_content_length_is_ignored() {
        let bytes = Response::new(
            200,
            vec![
                ("Content-Length", "999"),
                ("X-Test", "yes"),
            ],
            b"abc".to_vec(),
        )
        .to_bytes();
        let response = String::from_utf8(bytes).unwrap();

        assert!(response.contains("X-Test: yes\r\n"));
        assert!(response.contains("Content-Length: 3\r\n"));
        assert!(!response.contains("Content-Length: 999\r\n"));
        assert_eq!(response.matches("Content-Length:").count(), 1);
    }
}
