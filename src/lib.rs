use std::{
    io::{Read, Write},
    net::TcpStream,
};

use http::{
    request::Request,
    response::{Response, StatusCode},
};

pub mod http;

pub fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
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

    println!("Request: {request:#?}");

    let response = Response::html(StatusCode::Ok, "<h1>Hello World</h1>");

    stream.write_all(&response.to_bytes())?;
    Ok(())
}
