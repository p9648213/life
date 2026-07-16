use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::http::{
    request::Request,
    response::{Response, StatusCode},
    router::Router,
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

    pub fn read_tcp_stream(mut stream: &TcpStream) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;
        Ok(data)
    }

    pub fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let bytes_slice = Self::read_tcp_stream(&stream)?;
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
