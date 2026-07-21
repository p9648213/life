use std::collections::HashMap;

use crate::{
    constant::{CONTENT_LENGTH, CONTENT_TYPE, FORM_CONTENT_TYPE},
    http::error::HttpError,
    util::decode_form,
};

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Debug)]
struct ContentLength(usize);

#[derive(Debug)]
struct RequestLine<'buf> {
    method: HttpMethod,
    target_path: &'buf str,
    version: &'buf str,
}

#[derive(Debug)]
pub struct Request<'buf> {
    request_line: RequestLine<'buf>,
    headers: HashMap<String, &'buf str>,
    body: &'buf [u8],
    query: HashMap<&'buf str, &'buf str>,
    path: &'buf str,
}

impl<'buf> Request<'buf> {
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

    pub fn parse(data_buffer: &'buf [u8]) -> Result<Request<'buf>, HttpError> {
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
            return Err(HttpError::RequestHeaderInvalid);
        }
        let request_line = Self::parse_request_line(request_line)?;
        let (content_length, headers_map) = Self::parse_header(headers)?;
        if request_line.version != "HTTP/1.1" {
            return Err(HttpError::RequestHttpVersionInvalid);
        }
        let parse_query = Self::parse_query(request_line.target_path)?;
        let body = match content_length {
            Some(ref content_length) => {
                if content_length.0 == data_buffer.len() - body_start_index {
                    &data_buffer[body_start_index..]
                } else {
                    return Err(HttpError::ContentLengthSizeError);
                }
            }
            None => {
                if body_start_index != data_buffer.len() {
                    return Err(HttpError::RequestHeaderInvalid);
                }
                &[]
            }
        };
        Ok(Self {
            request_line,
            headers: headers_map,
            body,
            query: parse_query.1,
            path: parse_query.0,
        })
    }

    fn parse_method(method: &str) -> Result<HttpMethod, HttpError> {
        match method {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            _ => Err(HttpError::MethodParseError),
        }
    }

    fn parse_request_line(request_line_bytes: &'buf [u8]) -> Result<RequestLine<'buf>, HttpError> {
        let rl = str::from_utf8(request_line_bytes)?;
        let mut rl_iter = rl.split_whitespace();
        let method = Self::parse_method(rl_iter.next().unwrap_or_default())?;
        let target_path = rl_iter.next().unwrap_or_default();
        let version = rl_iter.next().unwrap_or_default();
        if rl_iter.next().is_some() || target_path.is_empty() || version.is_empty() {
            Err(HttpError::RequestLineInvalid)
        } else {
            Ok(RequestLine {
                method,
                target_path,
                version,
            })
        }
    }

    fn parse_header(
        header_bytes: &'buf [u8],
    ) -> Result<(Option<ContentLength>, HashMap<String, &'buf str>), HttpError> {
        let headers = str::from_utf8(header_bytes)?;
        let headers_iters = headers.split("\r\n");
        let mut headers_map = HashMap::new();
        let mut content_length = None;
        for header in headers_iters {
            if let Some((name, value)) = header.split_once(":") {
                let name = name.trim();
                let value = value.trim();
                if name.is_empty() {
                    return Err(HttpError::RequestHeaderInvalid);
                }
                headers_map.insert(name.to_ascii_uppercase(), value);
                if name.eq_ignore_ascii_case(CONTENT_LENGTH) {
                    content_length = Some(ContentLength(value.parse::<usize>()?));
                }
            } else {
                return Err(HttpError::RequestHeaderInvalid);
            }
        }
        Ok((content_length, headers_map))
    }

    fn parse_query(target_path: &str) -> Result<(&str, HashMap<&str, &str>), HttpError> {
        let mut query = HashMap::new();
        let mut path = target_path;
        let split_query = target_path.split_once("?");
        if let Some(q) = split_query {
            path = q.0;
            let query_params = q.1.split("&");
            for param in query_params {
                let kv = param.split_once("=");
                if let Some((key, value)) = kv
                    && !key.is_empty()
                {
                    query.insert(key, value);
                } else {
                    return Err(HttpError::RequestRouteInvalid);
                }
            }
        } else {
            return Ok((path, query));
        }

        Ok((path, query))
    }

    fn parse_form(&self) -> Result<HashMap<String, String>, HttpError> {
        if let Some(content_type) = self.get_header(CONTENT_TYPE) {
            let mut content_type = content_type.split(";");
            if let Some(content_type) = content_type.next()
                && content_type.trim().eq_ignore_ascii_case(FORM_CONTENT_TYPE)
            {
                let body_utf8 = str::from_utf8(self.body())?.trim();
                let mut form_map = HashMap::new();
                let name_value_slice = body_utf8.split("&");
                for name_value in name_value_slice {
                    match name_value.split_once("=") {
                        Some((name, value)) => {
                            if name.is_empty() {
                                return Err(HttpError::FormFieldMissingName);
                            }
                            if value.is_empty() {
                                return Err(HttpError::FormFieldMissingValue);
                            }
                            let decoded_name = decode_form(name.as_bytes())?;
                            let decoded_value = decode_form(value.as_bytes())?;
                            form_map.insert(decoded_name, decoded_value);
                        }
                        None => return Err(HttpError::FormParseError),
                    }
                }
                Ok(form_map)
            } else {
                Err(HttpError::FormParseError)
            }
        } else {
            Err(HttpError::FormParseError)
        }
    }

    pub fn extract_form<const N: usize>(
        &self,
        fields: [&str; N],
    ) -> Result<[String; N], HttpError> {
        let mut form_map = self.parse_form()?;
        let mut values = Vec::with_capacity(N);
        for field in fields {
            match form_map.remove(field) {
                Some(value) => values.push(value),
                None => {
                    return Err(HttpError::FormMissingField(field.to_owned()));
                }
            }
        }
        Ok(values.try_into().unwrap())
    }
}
