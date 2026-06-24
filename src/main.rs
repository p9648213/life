use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    request::Request,
    response::{Response, StatusCode},
};

mod error;
mod request;
mod response;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0u8; 256];
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

    println!("Request: {:#?}", request);

    let response = Response::html(StatusCode::Ok, "<h1>Hello World</h1>");

    stream.write_all(&response.to_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Bound to port 8080");
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::handle_client;
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream},
        thread,
    };

    #[test]
    fn handle_client_writes_http_response_over_tcp() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();

        let server = thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_client(stream).unwrap();
        });

        let mut client = TcpStream::connect(address).unwrap();
        client
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .unwrap();

        let mut response = String::new();
        client.read_to_string(&mut response).unwrap();
        server.join().unwrap();

        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(response.contains("Content-Type: text/html; charset=utf-8\r\n"));
        assert!(response.ends_with("<h1>Hello World</h1>"));
    }

    #[test]
    fn handle_client_returns_400_for_malformed_request() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();

        let server = thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            handle_client(stream).unwrap();
        });

        let mut client = TcpStream::connect(address).unwrap();
        client.write_all(b"this is not HTTP\r\n\r\n").unwrap();

        let mut response = String::new();
        client.read_to_string(&mut response).unwrap();
        server.join().unwrap();

        assert!(response.starts_with("HTTP/1.1 400 Bad Request\r\n"));
    }
}
