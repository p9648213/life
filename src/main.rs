use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

mod response;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0u8; 128]; 
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        return Ok(());
    }

    let data = &buffer[..bytes_read];

    println!("Request: {}", String::from_utf8_lossy(data));

    let body = "Hello world!";

    let response = format!("HTTP/1.1 200 OK\r\n\
    Content-Length: {}\r\n\
    Content-Type: text/plain\r\n\
    Connection: close\r\n\
    \r\n\
    {}", body.len(), &body);

    stream.write_all(response.as_bytes())?;
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
