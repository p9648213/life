# Phase 01: TCP Server

Goal: prove that a web server starts as a program listening on a TCP port.

This phase is intentionally low-level. You are not building a clean HTTP abstraction yet. You are learning how a Rust process accepts network connections and sends bytes back.

## What to Learn

- `TcpListener` listens for incoming TCP connections.
- `TcpStream` represents one connected client.
- `bind` attaches your program to an address and port.
- `accept` waits for a client.
- `read` receives bytes.
- `write_all` sends bytes.
- HTTP is data sent over a TCP stream.

## Where to Look

Official Rust docs:

- `std::net`: https://doc.rust-lang.org/std/net/
- `TcpListener`: https://doc.rust-lang.org/std/net/struct.TcpListener.html
- `TcpStream`: https://doc.rust-lang.org/std/net/struct.TcpStream.html
- `std::io::Read`: https://doc.rust-lang.org/std/io/trait.Read.html
- `std::io::Write`: https://doc.rust-lang.org/std/io/trait.Write.html

Background:

- MDN HTTP overview: https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview
- MDN HTTP messages: https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages

## Understanding the Target

When you see:

```text
Bind a TcpListener to 127.0.0.1:8080
```

It means:

- `127.0.0.1` is the loopback address. Only your own machine can connect.
- `8080` is the port number. Your program reserves that port while running.
- `TcpListener::bind(...)` asks the operating system to start listening there.
- If another process already uses port `8080`, binding fails.

The mental model:

```text
your Rust process
  asks OS: "please listen on 127.0.0.1:8080"

browser or curl
  connects to 127.0.0.1:8080

OS
  gives your Rust process a TcpStream for that client
```

## Tiny Pseudocode Shape

Do not copy this as finished code. Use it as a map:

```text
import TcpListener
import Read and Write traits

create listener at "127.0.0.1:8080"

wait for one connection
when connection arrives:
  read bytes into buffer
  print the bytes
  write a valid HTTP response
```

## Important Rust Clues

You will probably need imports like these concepts:

```text
std::net::TcpListener
std::io::Read
std::io::Write
```

Why?

- `TcpListener` gives you `bind` and `accept`.
- `Read` makes `read` available on the stream.
- `Write` makes `write_all` available on the stream.

When Rust says a method does not exist, check whether the trait that provides the method is imported.

## Step-by-Step Work

1. Open the `TcpListener` documentation.
2. Find the `bind` method.
3. Look at its return type.
4. Notice it returns a `Result`.
5. Decide how you want to handle failure for now. For learning, `expect(...)` is acceptable at this stage.
6. Bind to `127.0.0.1:8080`.
7. Print a message after binding succeeds.
8. Accept one connection.
9. Print the peer address if you can find the method for it.
10. Create a byte buffer, for example a fixed array.
11. Read from the stream into the buffer.
12. Convert only the bytes that were actually read into text for printing.
13. Write back a tiny HTTP response.

## Minimal Valid HTTP Response

Your first response should look conceptually like this:

```text
HTTP/1.1 200 OK\r\n
Content-Length: 12\r\n
Content-Type: text/plain\r\n
\r\n
Hello world!
```

The blank line between headers and body is required.

Important details:

- Lines in HTTP use `\r\n`, not just `\n`.
- `Content-Length` is the number of body bytes.
- The header section ends with an empty line.

## Manual Tests

Run the server:

```bash
cargo run
```

In another terminal:

```bash
curl -v http://127.0.0.1:8080/
```

Then try a browser:

```text
http://127.0.0.1:8080/
```

## Things to Observe

Look at the raw request your program prints. You should see something like:

```text
GET / HTTP/1.1
Host: 127.0.0.1:8080
User-Agent: ...
```

Do not parse it yet. Just observe it.

## Common Problems

Problem: port already in use.

What it means:

- Another process is already listening on `8080`.

What to try:

- Stop the other server.
- Or temporarily use `127.0.0.1:8081`.

Problem: browser keeps loading.

Possible causes:

- Response is missing the blank line between headers and body.
- `Content-Length` is wrong.
- You did not write the response to the stream.

Problem: Rust says `read` or `write_all` does not exist.

Possible cause:

- You forgot to import the `Read` or `Write` trait.

## Questions to Answer

- What is the difference between `TcpListener` and `TcpStream`?
- Why does `bind` return a `Result`?
- Why do you read into a byte buffer instead of directly into a `String`?
- What exactly does `Content-Length` count?
- What changes when you use `127.0.0.1:8081` instead?

## Checkpoint

You are done when:

- Your server listens on `127.0.0.1:8080`.
- `curl` can connect.
- Your program prints the raw request.
- Your program sends a valid response.
- You can explain each line of the response.

