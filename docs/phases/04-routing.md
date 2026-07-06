# Phase 04: Routing

Goal: map a parsed request to the correct behavior.

Routing is the layer between HTTP parsing and application logic. It should not know about `TcpStream`.

## What to Learn

- Exact route matching
- Method matching
- Handler functions
- Separating the path from the query string
- `404 Not Found`

## Where to Look

- MDN request methods: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
- MDN 404: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404
- Rust pattern matching: https://doc.rust-lang.org/book/ch06-02-match.html

## Routing Scope

This backend intentionally starts with exact static paths. It does not match dynamic path patterns such as `/resources/:id`.

Use query parameters when a handler needs a resource identifier or another selector:

```text
GET  /resources?id=123
POST /resources/delete?id=123
```

The router matches only the method and path, such as `GET /resources`. The handler reads and validates `id` from the parsed query parameters. The raw request target, normalized path, and query parameters should remain distinct concepts in `Request`.

This is a deliberate API constraint, not a claim that query parameters and dynamic path segments have identical semantics. Exact paths keep route matching simple. Reconsider dynamic paths only if a real application needs hierarchical resource URLs or another requirement that static paths cannot express cleanly.

Before depending heavily on queries, define the supported query format. Empty values, repeated keys, percent-encoding, `+`, and malformed pairs all need explicit behavior and tests.

## Start With Exact Routes

Support:

```text
GET  /
GET  /health
```

Later:

```text
POST /demo/form
GET  /resources?id=123
POST /resources/delete?id=123
```

These are sample routes for exercising the backend. Replace them when your real app domain exists.

## Step-by-Step Work

1. Expose the parsed method and target path through small `Request` accessors.
2. Create a function that receives a parsed request and returns a response.
3. Match on method and path.
4. Return a response for each route.
5. Return `404` when the path is unknown.
6. Keep query parsing separate from route lookup.
7. Move route-specific behavior into handler functions when the match grows.

## Tiny Pseudocode Shape

```text
route(request):
  if method is GET and path is "/":
    return home response

  if method is GET and path is "/health":
    return health response

  return 404
```

## Design Boundary

Try to keep these roles separate:

- Request parser understands bytes and HTTP message format.
- Router understands method and the exact path, without the query string.
- Handler understands app behavior and validates query values it needs.
- Response builder understands HTTP response formatting.
- Connection code understands `TcpStream`.

## Experiments

```bash
curl -i http://127.0.0.1:8080/
curl -i http://127.0.0.1:8080/health
curl -i 'http://127.0.0.1:8080/resources?id=123'
curl -i http://127.0.0.1:8080/not-real
curl -i -X POST http://127.0.0.1:8080/
```

## Questions to Answer

- Should the router know about TCP streams?
  - No. Connection code owns reading and writing; the router consumes a parsed request and returns a response.
- Should a handler manually write bytes to the stream?
  - No. A handler returns a response.
- Should route matching allocate strings?
  - It should be able to borrow the parsed path. Query storage may allocate separately, but route lookup does not need to construct a new path string.
- Why are dynamic path segments excluded?
  - Static paths plus explicit query parameters keep matching simple for the current backend API. This can be revisited when an application requirement justifies the extra routing complexity.

## Checkpoint

You are done when:

- Different paths return different responses.
- Unknown paths return 404.
- Query-bearing GET and POST targets match using the path without the query string.
- Query parsing behavior is covered for valid, empty, repeated, encoded, and malformed values according to the supported subset.
- Router tests can run without opening a TCP socket.
