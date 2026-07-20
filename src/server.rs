use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    constant::{MAX_BUFFER_SIZE, MAX_REQUEST_BYTES},
    http::{
        request::Request,
        response::{Response, StatusCode},
        router::Router,
    },
};

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
            let mut buffer = [0u8; MAX_BUFFER_SIZE];
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                if let Some(expected_length) = expected_length
                    && data.len() < expected_length
                {
                    return Err(Error::other("Request body incomplete"));
                }
                if expected_length.is_none() {
                    return Err(Error::other("Partial header"));
                }
                break;
            }
            if let Some(expected_length) = expected_length
                && expected_length - data.len() <= MAX_BUFFER_SIZE
                && expected_length - data.len() <= bytes_read
            {
                data.extend_from_slice(&buffer[..(expected_length - data.len())]);
            } else {
                let remaining = MAX_REQUEST_BYTES - data.len();
                let bytes_after_extend = data
                    .len()
                    .checked_add(bytes_read)
                    .ok_or_else(|| Error::other("Request size overflow"))?;
                if bytes_after_extend > MAX_REQUEST_BYTES {
                    data.extend_from_slice(&buffer[..remaining]);
                } else {
                    data.extend_from_slice(&buffer[..bytes_read]);
                }
            }
            if expected_length.is_none() {
                let mut index = 0;
                while index < data.len() && expected_length.is_none() {
                    if data.get(index..index + 4) == Some(&[13, 10, 13, 10]) {
                        let header = str::from_utf8(&data[..index])
                            .map_err(|err| Error::other(err.to_string()))?;
                        for (line_index, line) in header.lines().enumerate() {
                            if line_index == 0 {
                                continue;
                            }
                            if let Some((name, value)) = line.split_once(":")
                                && name.trim().eq_ignore_ascii_case("Content-Length")
                            {
                                let value = value
                                    .trim()
                                    .parse::<usize>()
                                    .map_err(|err| Error::other(err.to_string()))?;
                                let sum = value
                                    .checked_add(index + 4)
                                    .ok_or_else(|| Error::other("Request size overflow"))?;
                                expected_length = Some(sum);
                            }
                        }
                        if expected_length.is_none() {
                            expected_length = Some(index + 4);
                        }
                        if let Some(expected_length) = expected_length
                            && expected_length > MAX_REQUEST_BYTES
                        {
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
        for incomming in listener.incoming() {
            let stream = incomming?;
            if let Err(error) = self.handle_client(stream) {
                eprintln!("Client connection error: {error}");
            }
        }
        Ok(())
    }
}
