use std::collections::HashMap;

use crate::http::error::AppError;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
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
    headers: HashMap<String, &'a str>,
    body: &'a [u8],
    query: HashMap<&'a str, &'a str>,
    path: &'a str
}

//GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\n\r\n

impl<'a> Request<'a> {
    pub fn method(&self) -> &HttpMethod {
        &self.request_line.method
    }

    pub fn target_path(&self) -> &str {
        self.request_line.target_path
    }

    pub fn version(&self) -> &str {
        self.request_line.version
    }

    pub fn body(&self) -> &[u8] {
        self.body
    }

    pub fn query(&self) -> &HashMap<&str, &str> {
        &self.query
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers.get(&name.to_ascii_uppercase()).copied()
    }

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

        let parse_query = Self::parse_query(request_line.target_path)?;

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
            body,
            query: parse_query.1,
            path: parse_query.0
        })
    }

    fn parse_method(method: &str) -> Result<HttpMethod, AppError> {
        match method {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
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

    fn parse_query(target_path: &str) -> Result<(&str ,HashMap<&str, &str>), AppError> {
        let mut query = HashMap::new();
        let mut path = target_path;

        let split_query = target_path.rsplit_once("?");

        if split_query.iter().count() > 1 {
            return Err(AppError::RequestRouteInvalid)
        }

        if let Some(q) = split_query {
            path = q.0;
            let query_params = q.1.split("&");
            for param in query_params {
                let kv = param.split_once("=");
                if let Some((key, value)) = kv {
                    query.insert(key, value);
                } else {
                    return Err(AppError::RequestRouteInvalid);
                }
            }
        } else {
            return Ok((path, query))
        }

        Ok((path, query))
    }
}
