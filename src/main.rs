use std::{io::Read, net::{TcpListener, TcpStream}};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 128]; 
    let bytes_read = stream.read(&mut buffer).unwrap();

    if bytes_read == 0 {
        return;
    }

    let data = &buffer[..bytes_read];
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Binded to port 8080");
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
