use life::http::{
    request::Request,
    response::{Response, StatusCode},
    router::Router,
};

fn home<'a>(_: &'a Request<'_>) -> Response<'a> {
    Response::html(StatusCode::Ok, "<h1>Home</h1>")
}

fn health<'a>(_: &'a Request<'_>) -> Response<'a> {
    Response::html(StatusCode::Ok, "<h1>Healthy</h1>")
}

fn post_health<'a>(_: &'a Request<'_>) -> Response<'a> {
    Response::html(StatusCode::Ok, "<h1>Posted Health</h1>")
}

fn form<'a>(_: &'a Request<'_>) -> Response<'a> {
    Response::html(StatusCode::Ok, "<h1>Form</h1>")
}

fn parse_ok(data: &[u8]) -> Request<'_> {
    Request::parse(data).expect("request should parse")
}

fn response_text(response: Response<'_>) -> String {
    String::from_utf8(response.to_bytes()).expect("response should be valid UTF-8")
}

#[test]
fn handle_request_routes_different_paths_without_tcp_socket() {
    let mut router = Router::new();
    router.get("/", home);
    router.get("/health", health);

    let home_request = parse_ok(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
    let health_request = parse_ok(b"GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n");

    let home_response = response_text(router.handle_request(&home_request));
    let health_response = response_text(router.handle_request(&health_request));

    assert!(home_response.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(home_response.ends_with("<h1>Home</h1>"));
    assert!(health_response.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(health_response.ends_with("<h1>Healthy</h1>"));
}

#[test]
fn handle_request_uses_method_when_matching_routes() {
    let mut router = Router::new();
    router.get("/health", health);
    router.post("/health", post_health);

    let get_request = parse_ok(b"GET /health HTTP/1.1\r\nHost: localhost\r\n\r\n");
    let post_request =
        parse_ok(b"POST /health HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n");

    let get_response = response_text(router.handle_request(&get_request));
    let post_response = response_text(router.handle_request(&post_request));

    assert!(get_response.ends_with("<h1>Healthy</h1>"));
    assert!(post_response.ends_with("<h1>Posted Health</h1>"));
}

#[test]
fn handle_request_returns_404_for_unknown_path() {
    let mut router = Router::new();
    router.get("/", home);

    let request = parse_ok(b"GET /not-real HTTP/1.1\r\nHost: localhost\r\n\r\n");
    let response = response_text(router.handle_request(&request));

    assert!(response.starts_with("HTTP/1.1 404 Not Found\r\n"));
    assert!(response.ends_with("<h1>404 Not Found</h1>"));
}

#[test]
fn handle_request_returns_404_when_path_exists_for_different_method() {
    let mut router = Router::new();
    router.get("/health", health);

    let request =
        parse_ok(b"POST /health HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n");
    let response = response_text(router.handle_request(&request));

    assert!(response.starts_with("HTTP/1.1 404 Not Found\r\n"));
    assert!(response.ends_with("<h1>404 Not Found</h1>"));
}

#[test]
fn handle_request_matches_get_route_using_path_without_query() {
    let mut router = Router::new();
    router.get("/health", health);

    let request = parse_ok(b"GET /health?check=1 HTTP/1.1\r\nHost: localhost\r\n\r\n");
    let response = response_text(router.handle_request(&request));

    assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(response.ends_with("<h1>Healthy</h1>"));
}

#[test]
fn handle_request_matches_post_route_using_path_without_query() {
    let mut router = Router::new();
    router.post("/demo/form", form);

    let request = parse_ok(
        b"POST /demo/form?source=test HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n",
    );
    let response = response_text(router.handle_request(&request));

    assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(response.ends_with("<h1>Form</h1>"));
}
