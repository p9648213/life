use std::{
    collections::VecDeque,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use life::{
    constant::{MAX_BUFFER_SIZE, MAX_REQUEST_BYTES},
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    server::Server,
};

fn hello_world<'a>(_: &'a Request<'_>) -> Response<'a> {
    Response::html(StatusCode::Ok, "<h1>HELLO WORLD</h1>")
}

fn post_request_parts_for_total_length(total_length: usize) -> (String, usize) {
    let mut body_length = total_length;

    loop {
        let headers = format!("POST / HTTP/1.1\r\nContent-Length: {body_length}\r\n\r\n");
        let adjusted_body_length = total_length
            .checked_sub(headers.len())
            .expect("request limit must be large enough to contain POST headers");

        if adjusted_body_length == body_length {
            return (headers, body_length);
        }

        body_length = adjusted_body_length;
    }
}

struct ChunkReader {
    chunks: VecDeque<Vec<u8>>,
}

impl ChunkReader {
    fn new(chunks: Vec<Vec<u8>>) -> Self {
        Self {
            chunks: chunks.into(),
        }
    }

    fn remaining_chunks(&self) -> usize {
        self.chunks.len()
    }
}

impl Read for ChunkReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let Some(mut chunk) = self.chunks.pop_front() else {
            return Ok(0);
        };

        let bytes_read = chunk.len().min(buffer.len());
        let unread = chunk.split_off(bytes_read);
        buffer[..bytes_read].copy_from_slice(&chunk);

        if !unread.is_empty() {
            self.chunks.push_front(unread);
        }

        Ok(bytes_read)
    }
}

#[test]
fn reads_body_when_headers_and_body_arrive_separately() {
    let headers = b"POST / HTTP/1.1\r\nContent-Length: 3\r\n\r\n";
    let body = b"abc";
    let mut reader = ChunkReader::new(vec![
        headers.to_vec(),
        body.to_vec(),
        b"must stay unread".to_vec(),
    ]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(request, [headers.as_slice(), body.as_slice()].concat());
    assert_eq!(reader.remaining_chunks(), 1);
}

#[test]
fn counts_body_bytes_that_arrive_with_headers() {
    let first_chunk = b"POST / HTTP/1.1\r\nContent-Length: 3\r\n\r\na";
    let second_chunk = b"bc";
    let mut reader = ChunkReader::new(vec![first_chunk.to_vec(), second_chunk.to_vec()]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(
        request,
        [first_chunk.as_slice(), second_chunk.as_slice()].concat()
    );
}

#[test]
fn keeps_only_declared_request_when_suffix_arrives_in_first_read() {
    let expected_request = b"POST / HTTP/1.1\r\nContent-Length: 3\r\n\r\nabc";
    let mut first_chunk = expected_request.to_vec();
    first_chunk.extend_from_slice(b"extra");
    let mut reader = ChunkReader::new(vec![first_chunk]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(request, expected_request);
}

#[test]
fn finds_header_terminator_split_across_reads() {
    let first_chunk = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r";
    let second_chunk = b"\n";
    let mut reader = ChunkReader::new(vec![first_chunk.to_vec(), second_chunk.to_vec()]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(
        request,
        [first_chunk.as_slice(), second_chunk.as_slice()].concat()
    );
}

#[test]
fn bodyless_request_completes_at_end_of_headers() {
    let request_bytes = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let mut reader = ChunkReader::new(vec![request_bytes.to_vec(), b"unexpected".to_vec()]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(request, request_bytes);
    assert_eq!(reader.remaining_chunks(), 1);
}

#[test]
fn content_length_zero_completes_at_end_of_headers() {
    let request_bytes = b"POST / HTTP/1.1\r\nContent-Length: 0\r\n\r\n";
    let mut reader = ChunkReader::new(vec![request_bytes.to_vec(), b"unexpected".to_vec()]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(request, request_bytes);
    assert_eq!(reader.remaining_chunks(), 1);
}

#[test]
fn content_length_is_case_insensitive_while_accumulating() {
    let headers = b"POST / HTTP/1.1\r\ncontent-length: 3\r\n\r\n";
    let body = b"abc";
    let mut reader = ChunkReader::new(vec![headers.to_vec(), body.to_vec()]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(request, [headers.as_slice(), body.as_slice()].concat());
}

#[test]
fn rejects_whitespace_before_content_length_colon_while_accumulating() {
    let mut reader = ChunkReader::new(vec![
        b"POST / HTTP/1.1\r\nContent-Length : 3\r\n\r\nabc".to_vec(),
    ]);

    assert!(Server::read_one_request(&mut reader).is_err());
}

#[test]
fn rejects_eof_before_request_head_is_complete() {
    let mut reader = ChunkReader::new(vec![b"GET / HTTP/1.1\r\nHost: localhost\r\n".to_vec()]);

    assert!(Server::read_one_request(&mut reader).is_err());
}

#[test]
fn rejects_eof_before_declared_body_is_complete() {
    let mut reader = ChunkReader::new(vec![
        b"POST / HTTP/1.1\r\nContent-Length: 5\r\n\r\nabc".to_vec(),
    ]);

    assert!(Server::read_one_request(&mut reader).is_err());
}

#[test]
fn rejects_invalid_content_length_without_panicking() {
    let mut reader = ChunkReader::new(vec![
        b"POST / HTTP/1.1\r\nContent-Length: nope\r\n\r\n".to_vec(),
    ]);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Server::read_one_request(&mut reader)
    }));

    assert!(result.is_ok(), "invalid request input must not panic");
    assert!(result.unwrap().is_err());
}

#[test]
fn rejects_non_utf8_headers_without_panicking() {
    let mut reader = ChunkReader::new(vec![b"GET / HTTP/1.1\r\nX-Name: \xff\r\n\r\n".to_vec()]);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Server::read_one_request(&mut reader)
    }));

    assert!(result.is_ok(), "invalid request input must not panic");
    assert!(result.unwrap().is_err());
}

#[test]
fn rejects_content_length_larger_than_usize_without_panicking() {
    let overflowing_length = format!("{}0", usize::MAX);
    let request = format!("POST / HTTP/1.1\r\nContent-Length: {overflowing_length}\r\n\r\n");
    let mut reader = ChunkReader::new(vec![request.into_bytes()]);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Server::read_one_request(&mut reader)
    }));

    assert!(result.is_ok(), "invalid request input must not panic");
    assert!(result.unwrap().is_err());
}

#[test]
fn rejects_content_length_when_total_length_overflows_usize_without_panicking() {
    let request = format!("POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n", usize::MAX);
    let mut reader = ChunkReader::new(vec![request.into_bytes()]);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Server::read_one_request(&mut reader)
    }));

    assert!(result.is_ok(), "request length calculation must not panic");
    assert!(result.unwrap().is_err());
}

#[test]
fn does_not_count_bytes_after_first_request_toward_request_capacity() {
    const REQUIRED_BYTES_IN_FINAL_READ: usize = 5;

    let (headers, body_length) = post_request_parts_for_total_length(MAX_REQUEST_BYTES);
    assert_eq!(headers.len() + body_length, MAX_REQUEST_BYTES);

    let body_before_final_read = vec![b'a'; body_length - REQUIRED_BYTES_IN_FINAL_READ];
    let mut chunks = vec![headers.as_bytes().to_vec()];
    chunks.extend(
        body_before_final_read
            .chunks(MAX_BUFFER_SIZE)
            .map(|chunk| chunk.to_vec()),
    );

    let mut final_chunk = vec![b'a'; REQUIRED_BYTES_IN_FINAL_READ];
    final_chunk.extend_from_slice(b"extra");
    chunks.push(final_chunk);

    let mut reader = ChunkReader::new(chunks);
    let request = Server::read_one_request(&mut reader)
        .expect("bytes after the first request must not count toward its capacity");

    assert_eq!(request.len(), MAX_REQUEST_BYTES);
    assert_eq!(&request[..headers.len()], headers.as_bytes());
    assert!(request[headers.len()..].iter().all(|byte| *byte == b'a'));
}

#[test]
fn keeps_exact_capacity_request_when_boundary_read_also_contains_suffix() {
    const HEADER_END: &[u8] = b"\r\n\r\n";

    let header_start = b"GET / HTTP/1.1\r\nX-Pad: ";
    let padding_length = MAX_REQUEST_BYTES - header_start.len() - HEADER_END.len();
    let mut expected_request = header_start.to_vec();
    expected_request.resize(expected_request.len() + padding_length, b'a');
    expected_request.extend_from_slice(HEADER_END);
    assert_eq!(expected_request.len(), MAX_REQUEST_BYTES);

    let final_boundary_start = expected_request.len() - 2;
    let mut chunks: Vec<Vec<u8>> = expected_request[..final_boundary_start]
        .chunks(MAX_BUFFER_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();
    let mut final_chunk = expected_request[final_boundary_start..].to_vec();
    final_chunk.extend_from_slice(b"extra");
    chunks.push(final_chunk);

    let mut reader = ChunkReader::new(chunks);
    let request = Server::read_one_request(&mut reader)
        .expect("suffix bytes must not make an exact-capacity request overflow");

    assert_eq!(request, expected_request);
}

#[test]
fn does_not_treat_content_length_text_in_request_line_as_header() {
    let malformed_head = b"Content-Length: 3\r\nHost: localhost\r\n\r\n";
    let mut reader = ChunkReader::new(vec![malformed_head.to_vec(), b"abc".to_vec()]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(request, malformed_head);
    assert_eq!(reader.remaining_chunks(), 1);
}

#[test]
fn does_not_treat_content_length_text_inside_the_body_as_a_header() {
    let body = b"Content-Length: 4";
    let headers = format!("POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n", body.len());
    let expected = [headers.as_bytes(), body.as_slice()].concat();
    let mut reader = ChunkReader::new(vec![expected.clone()]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(request, expected);
}

#[test]
fn does_not_treat_a_second_header_terminator_in_the_body_as_framing() {
    let body = b"a\r\n\r\nbcd";
    let headers = format!("POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n", body.len());
    let expected = [headers.as_bytes(), body.as_slice()].concat();
    let mut reader = ChunkReader::new(vec![expected.clone(), b"extra".to_vec()]);

    let request = Server::read_one_request(&mut reader).unwrap();

    assert_eq!(request, expected);
    assert_eq!(reader.remaining_chunks(), 1);
}

#[test]
fn rejects_header_that_fills_request_capacity_without_a_terminator() {
    let mut reader = ChunkReader::new(vec![vec![b'x'; MAX_REQUEST_BYTES]]);

    assert!(Server::read_one_request(&mut reader).is_err());
}

#[test]
fn rejects_declared_request_larger_than_capacity() {
    let request = format!("POST / HTTP/1.1\r\nContent-Length: {MAX_REQUEST_BYTES}\r\n\r\n");
    let mut reader = ChunkReader::new(vec![request.into_bytes()]);

    assert!(Server::read_one_request(&mut reader).is_err());
}

#[test]
fn handle_client_writes_http_response_over_tcp() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let mut app = Server::new();
    app.routes.get("/", hello_world);

    let server = thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        app.handle_client(stream).unwrap();
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
fn handle_client_returns_400_for_malformed_request() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let app = Server::new();

    let server = thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        app.handle_client(stream).unwrap();
    });

    let mut client = TcpStream::connect(address).unwrap();
    client.write_all(b"this is not HTTP\r\n\r\n").unwrap();

    let mut response = String::new();
    client.read_to_string(&mut response).unwrap();
    server.join().unwrap();

    assert!(response.starts_with("HTTP/1.1 400 Bad Request\r\n"));
}
