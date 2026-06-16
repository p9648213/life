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
}

//GET / HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nUser-Agent: curl/8.5.0\r\nAccept: */*\r\n\r\n

impl<'a> Request<'a> {
    pub fn parse(data_buffer: &[u8]) -> Self {
        println!("Buffer: {:?}", data_buffer);

        let mut first_line: &[u8] = &[];
        let mut carriage_return = false;

        for (index, byte) in data_buffer.iter().enumerate() {
            if *byte == 13 {
                carriage_return = true;
                continue;
            }

            if *byte == 10 && carriage_return {
                if first_line.is_empty() {
                    first_line = &data_buffer[..index - 1];
                    println!("First line byte: {:?}", first_line)
                }
                carriage_return = false;
                continue;
            }

            if carriage_return {
                carriage_return = false
            }
        }

        Self {
            method: HttpMethod::Get,
            target_path: "/",
            version: "1.1",
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
