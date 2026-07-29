use life::{
    http::{
        request::Request,
        response::{Response, StatusCode},
        router::Router,
    },
    state::State,
};

fn add_resource<'req>(_: &'req Request<'_>, state: &mut State) -> Response<'req> {
    state.resources.push("created".to_owned());
    Response::text_plain(StatusCode::Ok, "created")
}

fn list_resources<'req>(_: &'req Request<'_>, state: &mut State) -> Response<'req> {
    Response::text_plain(StatusCode::Ok, &state.resources.join(","))
}

fn parse_ok(data: &[u8]) -> Request<'_> {
    Request::parse(data).expect("request should parse")
}

#[test]
fn state_starts_empty() {
    let state = State::default();

    assert!(state.resources.is_empty());
}

#[test]
fn mutation_from_one_request_is_visible_to_a_later_request_without_tcp() {
    let mut router = Router::new();
    router.post("/resources", add_resource);
    router.get("/resources", list_resources);
    let mut state = State::default();

    let create_request =
        parse_ok(b"POST /resources HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n");
    let create_response = router.handle_request(&create_request, &mut state);

    assert!(
        create_response
            .to_bytes()
            .starts_with(b"HTTP/1.1 200 OK\r\n")
    );
    assert_eq!(state.resources, ["created"]);

    let list_request = parse_ok(b"GET /resources HTTP/1.1\r\nHost: localhost\r\n\r\n");
    let list_response = router.handle_request(&list_request, &mut state);
    let list_bytes = list_response.to_bytes();

    assert!(list_bytes.starts_with(b"HTTP/1.1 200 OK\r\n"));
    assert!(list_bytes.ends_with(b"created"));
    assert_eq!(state.resources, ["created"]);
}
