# Phase 04: Routing

Goal: map a parsed request to the correct behavior.

Routing is the layer between HTTP parsing and application logic.

## What to Learn

- Exact route matching
- Method matching
- Handler functions
- `404 Not Found`
- `405 Method Not Allowed`
- Dynamic path segments

## Where to Look

- MDN request methods: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
- MDN 404: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404
- MDN 405: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/405
- Rust pattern matching: https://doc.rust-lang.org/book/ch06-02-match.html

## Start With Exact Routes

Support:

```text
GET  /
GET  /notes
GET  /notes/new
POST /notes
```

Later:

```text
GET  /notes/:id
POST /notes/:id/delete
```

## Step-by-Step Work

1. Create a function that receives a parsed request.
2. Match on method and path.
3. Return a response for each route.
4. Return `404` when the path is unknown.
5. Return `405` when the path exists but the method is wrong.
6. Move route-specific behavior into handler functions when the match grows.

## Tiny Pseudocode Shape

```text
route(request):
  if method is GET and path is "/":
    return home response

  if method is GET and path is "/notes":
    return notes list response

  if path exists but method is wrong:
    return 405

  return 404
```

## Design Boundary

Try to keep these roles separate:

- Request parser understands bytes and HTTP message format.
- Router understands method and path.
- Handler understands app behavior.
- Response builder understands HTTP response formatting.

## Experiments

```bash
curl -i http://127.0.0.1:8080/
curl -i http://127.0.0.1:8080/notes
curl -i http://127.0.0.1:8080/notes/new
curl -i http://127.0.0.1:8080/not-real
curl -i -X POST http://127.0.0.1:8080/
```

## Questions to Answer

- Should the router know about TCP streams?
- Should a handler manually write bytes to the stream?
- How can you detect path exists but method is wrong?

## Checkpoint

You are done when:

- Different paths return different responses.
- Unknown paths return 404.
- Wrong methods return 405.

