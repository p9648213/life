use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::http::{
    request::Request,
    response::{Response, StatusCode},
    router::Router,
};

const MAX_REQUEST_BYTES: usize = 64 * 1024;

pub struct Server<'server> {
    pub routes: Router<'server>,
}

impl<'server> Default for Server<'server> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'server> Server<'server> {
    pub fn new() -> Self {
        Self {
            routes: Router::new(),
        }
    }

    pub fn read_one_request(reader: &mut impl Read) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        let mut expected_length = None;
        loop {
            let mut buffer = [0u8; 512];
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            let bytes_after_extend = data
                .len()
                .checked_add(bytes_read)
                .ok_or_else(|| Error::other("Request size overflow"))?;
            if bytes_after_extend > MAX_REQUEST_BYTES {
                return Err(Error::other("Request size overflow."));
            }
            data.extend_from_slice(&buffer[..bytes_read]);
            if expected_length.is_none() {
                let mut index = 0;
                while index < data.len() && expected_length.is_none() {
                    if data.get(index..index + 4) == Some(&[13, 10, 13, 10]) {
                        let header = str::from_utf8(&data[..index]).unwrap();
                        for line in header.lines() {
                            if let Some((name, value)) = line.split_once(":")
                                && name.eq_ignore_ascii_case("Content-Length")
                            {
                                let value = value.trim().parse::<usize>().unwrap();
                                expected_length = Some(value + index + 4);
                            }
                        }
                        if expected_length.is_none() {
                            expected_length = Some(index + 4);
                        }
                        if let Some(expected_length) = expected_length && expected_length > MAX_REQUEST_BYTES {
                            return Err(Error::other("Request size overflow"));
                        }
                    }
                    index += 1;
                }
                if expected_length.is_none() && data.len() == MAX_REQUEST_BYTES {
                    return Err(Error::other("Request size overflow"));
                }
            }
            if let Some(expected_length) = expected_length {
                if data.len() == expected_length {
                    break;
                }
                if data.len() > expected_length {
                    data.truncate(expected_length);
                    break;
                }
            }
        }
        Ok(data)
    }

    pub fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let bytes_slice = Self::read_one_request(&mut stream)?;
        let request = match Request::parse(&bytes_slice) {
            Ok(request) => request,
            Err(error) => {
                eprintln!("Bad request: {error}");
                let response = Response::html(StatusCode::BadRequest, "<h1>400 Bad Request</h1>");
                stream.write_all(&response.to_bytes())?;
                return Ok(());
            }
        };

        let response = self.routes.handle_request(&request);
        stream.write_all(&response.to_bytes())?;
        Ok(())
    }

    pub fn run(&self, address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        for stream in listener.incoming() {
            self.handle_client(stream?)?;
        }
        Ok(())
    }
}
