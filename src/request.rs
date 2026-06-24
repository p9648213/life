use std::collections::HashMap;

use crate::error::AppError;

#[derive(Debug, PartialEq)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug)]
struct ContentLength(usize);

#[derive(Debug)]
struct RequestLine<'a> {
    method: HttpMethod,
    target_path: &'a str,
    version: &'a str,
}

#[derive(Debug)]
pub struct Request<'a> {
    request_line: RequestLine<'a>,
    content_length: Option<ContentLength>,
    headers: HashMap<String, &'a str>,
    body: &'a [u8],
}

//GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\n\r\n

impl<'a> Request<'a> {
    pub fn parse(data_buffer: &'a [u8]) -> Result<Request<'a>, AppError> {
        let mut request_line: &[u8] = &[];
        let mut headers: &[u8] = &[];
        let mut index = 0;
        let mut body_start_index = 0;

        while index < data_buffer.len() {
            if request_line.is_empty() && data_buffer.get(index..index + 2) == Some(&[13, 10]) {
                request_line = &data_buffer[..index];
                index += 2;
            }

            if request_line.len() + 2 <= index
                && data_buffer.get(index..index + 4) == Some(&[13, 10, 13, 10])
            {
                headers = &data_buffer[request_line.len() + 2..index];
                body_start_index = index + 4;
                break;
            }

            index += 1;
        }

        if headers.is_empty() {
            return Err(AppError::RequestHeaderInvalid);
        }

        let request_line = Self::parse_request_line(request_line)?;
        let (content_length, headers_map) = Self::parse_header(headers)?;

        if request_line.version != "HTTP/1.1" {
            return Err(AppError::RequestHttpVersionInvalid);
        }

        let body = match content_length {
            Some(ref content_length) => {
                if content_length.0 == data_buffer.len() - body_start_index {
                    &data_buffer[body_start_index..]
                } else {
                    return Err(AppError::ContentLengthSizeError);
                }
            }
            None => {
                if body_start_index != data_buffer.len() {
                    return Err(AppError::RequestHeaderInvalid);
                }
                &[]
            }
        };

        Ok(Self {
            request_line,
            headers: headers_map,
            content_length,
            body,
        })
    }

    fn parse_method(method: &str) -> Result<HttpMethod, AppError> {
        match method {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "PATCH" => Ok(HttpMethod::Patch),
            "DELETE" => Ok(HttpMethod::Delete),
            _ => Err(AppError::MethodParseError),
        }
    }

    fn parse_request_line(request_line_bytes: &'a [u8]) -> Result<RequestLine<'a>, AppError> {
        let rl = str::from_utf8(request_line_bytes)?;
        let mut rl_iter = rl.split_whitespace();

        let method = Self::parse_method(rl_iter.next().unwrap_or_default())?;
        let target_path = rl_iter.next().unwrap_or_default();
        let version = rl_iter.next().unwrap_or_default();

        if rl_iter.next().is_some() || target_path.is_empty() || version.is_empty() {
            Err(AppError::RequestLineInvalid)
        } else {
            Ok(RequestLine {
                method,
                target_path,
                version,
            })
        }
    }

    fn parse_header(
        header_bytes: &'a [u8],
    ) -> Result<(Option<ContentLength>, HashMap<String, &'a str>), AppError> {
        let headers = str::from_utf8(header_bytes)?;
        let headers_iters = headers.split("\r\n");
        let mut headers_map = HashMap::new();
        let mut content_length = None;

        for header in headers_iters {
            if let Some((name, value)) = header.split_once(":") {
                let name = name.trim();
                let value = value.trim();

                if name.is_empty() {
                    return Err(AppError::RequestHeaderInvalid);
                }

                headers_map.insert(name.to_ascii_uppercase(), value);

                if name.eq_ignore_ascii_case("Content-Length") {
                    content_length = Some(ContentLength(value.parse::<usize>()?));
                }
            } else {
                return Err(AppError::RequestHeaderInvalid);
            }
        }

        Ok((content_length, headers_map))
    }

    fn get_header(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_ascii_uppercase()).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::{HttpMethod, Request};

    fn parse_ok(data: &[u8]) -> Request<'_> {
        Request::parse(data).expect("request should parse")
    }

    #[test]
    fn parses_valid_get_request() {
        let request = parse_ok(b"GET /notes HTTP/1.1\r\nHost: localhost\r\n\r\n");

        assert!(matches!(request.request_line.method, HttpMethod::Get));
        assert_eq!(request.request_line.target_path, "/notes");
        assert_eq!(request.request_line.version, "HTTP/1.1");
        assert_eq!(request.get_header("host"), Some("localhost"));
        assert_eq!(request.get_header("Host"), Some("localhost"));
        assert_eq!(request.get_header("HOST"), Some("localhost"));
        assert_eq!(request.body, b"");
    }

    #[test]
    fn parses_valid_post_body_using_content_length() {
        let request = parse_ok(
            b"POST /notes HTTP/1.1\r\nHost: localhost\r\nContent-Length: 22\r\n\r\ntitle=Hello&body=World",
        );

        assert!(matches!(request.request_line.method, HttpMethod::Post));
        assert_eq!(request.request_line.target_path, "/notes");
        assert_eq!(request.body, b"title=Hello&body=World");
    }

    #[test]
    fn preserves_colons_inside_header_values() {
        let request = parse_ok(b"GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\n\r\n");

        assert_eq!(request.get_header("HOST"), Some("127.0.0.1:8080"));
    }

    #[test]
    fn content_length_name_is_case_insensitive() {
        let request = parse_ok(b"POST / HTTP/1.1\r\ncontent-length: 3\r\n\r\nabc");

        assert_eq!(request.body, b"abc");
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
    fn rejects_surplus_body_bytes_because_pipelining_is_unsupported() {
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
}
