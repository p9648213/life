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
struct Header<'a> {
    name: &'a str,
    value: &'a str
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

        println!("First line: {:?}", first_line);
        println!("Header: {:?}", headers);
        println!("Body: {:?}", body);

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
