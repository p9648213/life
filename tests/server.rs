use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use life::{
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    server::Server,
};

fn hello_world<'a>(_: &'a Request<'_>) -> Response<'a> {
    Response::html(StatusCode::Ok, "<h1>HELLO WORLD</h1>")
}

#[test]
fn handle_tcp_buffer_writes_http_response_over_tcp() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let mut app = Server::new();
    app.routes.get("/", hello_world);

    let server = thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        app.handle_tcp_buffer(stream).unwrap();
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
    assert!(response.ends_with("<h1>HELLO WORLD</h1>"));
}

#[test]
fn handle_tcp_buffer_returns_400_for_malformed_request() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let app = Server::new();

    let server = thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        app.handle_tcp_buffer(stream).unwrap();
    });

    let mut client = TcpStream::connect(address).unwrap();
    client.write_all(b"this is not HTTP\r\n\r\n").unwrap();

    let mut response = String::new();
    client.read_to_string(&mut response).unwrap();
    server.join().unwrap();

    assert!(response.starts_with("HTTP/1.1 400 Bad Request\r\n"));
}
