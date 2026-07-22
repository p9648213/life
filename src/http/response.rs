use crate::{
    constant::{CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, TRANSFER_ENCODING},
    http::error::HttpError,
};

pub struct Response<'header> {
    status_code: StatusCode,
    headers: Vec<(&'header str, &'header str)>,
    body_bytes: Vec<u8>,
}

pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::BadRequest => 400,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
        }
    }

    pub fn reason_phrase(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
        }
    }
}

impl<'header> Response<'header> {
    pub fn new(status_code: StatusCode, body_bytes: Vec<u8>) -> Self {
        Self {
            status_code,
            headers: Vec::new(),
            body_bytes,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = Vec::new();
        response.extend_from_slice(
            format!(
                "HTTP/1.1 {} {}\r\n",
                self.status_code.as_u16(),
                self.status_code.reason_phrase()
            )
            .as_bytes(),
        );
        for header in &self.headers {
            if !header.0.eq_ignore_ascii_case(CONTENT_LENGTH) {
                response.extend_from_slice(format!("{}: {}\r\n", header.0, header.1).as_bytes());
            }
        }
        response.extend_from_slice(
            format!("{CONTENT_LENGTH}: {}\r\n", self.body_bytes.len()).as_bytes(),
        );
        response.extend_from_slice(b"Connection: close\r\n");
        response.extend_from_slice(b"\r\n");
        response.extend_from_slice(&self.body_bytes);
        response
    }

    pub fn add_header(&mut self, name: &'header str, value: &'header str) -> Result<(), HttpError> {
        for byte in [name, value].concat().bytes() {
            if byte == 13 || byte == 10 {
                return Err(HttpError::RequestHeaderInvalid);
            }
        }
        if name.eq_ignore_ascii_case(CONTENT_LENGTH)
            || name.eq_ignore_ascii_case(TRANSFER_ENCODING)
            || name.eq_ignore_ascii_case(CONNECTION)
        {
            return Err(HttpError::RequestAddInvalidHeader(name.to_owned()));
        }
        self.headers.push((name, value));
        Ok(())
    }

    pub fn set_header(&mut self, name: &'header str, value: &'header str) -> Result<(), HttpError> {
        for byte in [name, value].concat().bytes() {
            if byte == 13 || byte == 10 {
                return Err(HttpError::RequestHeaderInvalid);
            }
        }
        if name.eq_ignore_ascii_case(CONTENT_LENGTH)
            || name.eq_ignore_ascii_case(TRANSFER_ENCODING)
            || name.eq_ignore_ascii_case(CONNECTION)
        {
            return Err(HttpError::RequestAddInvalidHeader(name.to_owned()));
        }
        for (header_name, header_value) in self.headers.iter_mut() {
            if header_name.eq_ignore_ascii_case(name) {
                *header_value = value
            }
        }
        Ok(())
    }

    pub fn html(status_code: StatusCode, html: &str) -> Self {
        Response {
            status_code,
            headers: vec![(CONTENT_TYPE, "text/html; charset=utf-8")],
            body_bytes: html.as_bytes().to_vec(),
        }
    }

    pub fn text_plain(status_code: StatusCode, text: &str) -> Self {
        Response {
            status_code,
            headers: vec![(CONTENT_TYPE, "text/plain")],
            body_bytes: text.as_bytes().to_vec(),
        }
    }
}
