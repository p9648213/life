struct Response {
    status_code: u16,
    reason_phrase: String,
    headers: Vec<(String, String)>,
    body_bytes: Vec<u8>
}

impl Response {
    fn new(status_code: u16, headers: Vec<(String, String)>, body: &str) -> Self {
       Self {
            status_code,
            reason_phrase: String::from(Self::reason_phrase(status_code)),
            headers,
            body_bytes: body.as_bytes().to_vec()
       } 
    }

    fn reason_phrase(status_code: u16) -> &'static str {
        match status_code {
            200 => "Ok",
            404 => "Not Found",
            500 => "Interal Server Error",
            _ => "Unknown"
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        
    }
}
