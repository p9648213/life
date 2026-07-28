use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::http::{
    framing::read_one_request,
    request::Request,
    response::{Response, StatusCode},
    router::Router,
};

pub struct Server<'server, T> {
    pub routes: Router<'server, T>,
    pub state: T,
}

impl<'server, T> Server<'server, T> {
    pub fn new(state: T) -> Self {
        Self {
            routes: Router::new(),
            state,
        }
    }

    pub fn handle_client(&mut self, mut stream: TcpStream) -> std::io::Result<()> {
        let bytes_slice = read_one_request(&mut stream)?;
        let request = match Request::parse(&bytes_slice) {
            Ok(request) => request,
            Err(error) => {
                eprintln!("Bad request: {error}");
                let response = Response::html(StatusCode::BadRequest, "<h1>400 Bad Request</h1>");
                stream.write_all(&response.to_bytes())?;
                return Ok(());
            }
        };
        let response = self.routes.handle_request(&request, &mut self.state);
        stream.write_all(&response.to_bytes())?;
        Ok(())
    }

    pub fn run(&mut self, address: &str) -> std::io::Result<()> {
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
