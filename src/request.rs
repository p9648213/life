use std::collections::HashMap;

#[derive(Debug)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Unknown,
}

#[derive(Debug)]
pub struct Request<'a> {
    method: HttpMethod,
    target_path: &'a str,
    version: &'a str,
    headers: HashMap<&'a str, &'a str>,
    body: &'a [u8],
}

//GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\n\r\n

impl<'a> Request<'a> {
    pub fn parse(data_buffer: &'a [u8]) -> Self {
        println!("Buffer: {:?}", data_buffer);

        let mut first_line: &[u8] = &[];
        let mut headers: &[u8] = &[];
        let mut body: &[u8] = &[];

        let mut index = 0;

        while index < data_buffer.len() {
            if data_buffer[index] == 13 && data_buffer[index + 1] == 10 && first_line.is_empty() {
                first_line = &data_buffer[..index];
                index += 2;
            }

            if data_buffer[index] == 13
                && data_buffer[index + 2] == 13
                && data_buffer[index + 1] == 10
                && data_buffer[index + 3] == 10
            {
                headers = &data_buffer[first_line.len() + 2..index];
                body = &data_buffer[index + 4..data_buffer.len()];
                break;
            }

            index += 1;
        }

        let first_line = str::from_utf8(first_line).unwrap();
        let mut first_line_iter = first_line.split_whitespace();

        let headers = str::from_utf8(headers).unwrap();
        let headers_iters = headers.split("\r\n");
        let mut headers_map = HashMap::new();

        for header in headers_iters {
            let mut header_iters = header.split(":");
            let name = header_iters.next().unwrap().trim();
            let value = header_iters.next().unwrap().trim();
            headers_map.insert(name, value);
        }

        Self {
            method: Self::to_method(first_line_iter.next().unwrap()),
            target_path: first_line_iter.next().unwrap(),
            version: first_line_iter.next().unwrap(),
            headers: headers_map,
            body,
        }
    }

    fn to_method(method: &str) -> HttpMethod {
        match method {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "PATCH" => HttpMethod::Patch,
            "DELETE" => HttpMethod::Delete,
            _ => HttpMethod::Unknown,
        }
    }
}
