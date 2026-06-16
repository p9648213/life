# Phase 01 Feedback

## Overall

You completed the main goal of Phase 01: your program listens on a TCP port, accepts a connection, reads bytes from the client, prints the raw request, and sends back a valid HTTP response.

I verified that:

- `cargo check` passes.
- The server starts on `127.0.0.1:8080`.
- `curl -v http://127.0.0.1:8080/` receives `HTTP/1.1 200 OK`.
- The response body is `Hello world!`.
- The server prints the incoming HTTP request.

## What You Did Well

- You used `TcpListener` and `TcpStream` correctly.
- You imported the `Read` and `Write` traits so `read` and `write_all` are available.
- You read only the bytes that were actually received with `&buffer[..bytes_read]`.
- You used `String::from_utf8_lossy` instead of assuming the bytes are always valid UTF-8.
- You included the required blank line between HTTP headers and the body.
- You calculated `Content-Length` from the body instead of hard-coding it.
- You added `Connection: close`, which matches the simple server behavior.
- You extracted connection handling into `handle_client`, which keeps `main` easier to read.

## Things To Improve Later

- The buffer is only 128 bytes. That is fine for this phase, but real HTTP requests may be larger and may require multiple reads.
- `handle_client(stream?)?` means one connection error can stop the whole server. Later, you may want to log per-client errors and keep accepting new connections.
- The message `Binded to port 8080` should be `Bound to port 8080`.
- The response is built manually with `format!`. That is good for learning now, but Phase 02 will help you make response construction more deliberate.

## Answer Review

Your original answers had the right direction. The most important corrections were:

- `TcpStream` does not just capture data. It represents the active connection and lets both sides read and write bytes.
- `bind` returns `Result` because the operating system may refuse the bind request.
- Reading into a byte buffer matters because TCP gives raw bytes, not guaranteed valid text.
- `Content-Length` counts body bytes, not characters.
- Changing from `8080` to `8081` changes the port clients must connect to; the host is still local-only because it is `127.0.0.1`.

## Ready For Phase 02

You are ready to move on to the HTTP response builder phase. The main idea to carry forward is that an HTTP response is still just bytes written to a `TcpStream`, but now you will start organizing those bytes more carefully.
