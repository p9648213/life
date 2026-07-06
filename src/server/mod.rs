pub mod router;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    http::{
        request::{HttpMethod, Request},
        response::{Response, StatusCode},
    },
    server::router::Router,
};

pub struct Server<'a> {
    pub routes: Router<'a>,
}

impl<'a> Default for Server<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        Self {
            routes: Router::new(),
        }
    }

    pub fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {
        let mut buffer = [0u8; 512];
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            return Ok(());
        }

        let data = &buffer[..bytes_read];

        let request = match Request::parse(data) {
            Ok(request) => request,
            Err(error) => {
                eprintln!("Bad request: {error}");
                let response = Response::html(StatusCode::BadRequest, "<h1>400 Bad Request</h1>");
                stream.write_all(&response.to_bytes())?;
                return Ok(());
            }
        };

        match request.method() {
            HttpMethod::Get => {
                let routes = self.routes.get_routes();
                let path = request.path();

                if let Some(handler) = routes.get(path) {
                    let response = handler(&request);
                    stream.write_all(&response.to_bytes())?;
                }
            }
            HttpMethod::Post => {
                let routes = self.routes.post_routes();
                let path = request.target_path();

                if let Some(handler) = routes.get(path) {
                    let response = handler(&request);
                    stream.write_all(&response.to_bytes())?;
                }
            }
        }

        Ok(())
    }

    pub fn run(&self, address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        println!("Bound to {}", address);
        for stream in listener.incoming() {
            self.handle_client(stream?)?;
        }
        Ok(())
    }
}
