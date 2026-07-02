pub struct Response<'a> {
    status_code: StatusCode,
    headers: Vec<(&'a str, &'a str)>,
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

impl<'a> Response<'a> {
    pub fn new(
        status_code: StatusCode,
        headers: Vec<(&'a str, &'a str)>,
        body_bytes: Vec<u8>,
    ) -> Self {
        Self {
            status_code,
            headers,
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

    pub fn html(status_code: StatusCode, text: &str) -> Self {
        Self::new(
            status_code,
            vec![("Content-Type", "text/html; charset=utf-8")],
            text.as_bytes().to_vec(),
        )
    }
}
