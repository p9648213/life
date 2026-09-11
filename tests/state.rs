use life::http::{
    request::Request,
    response::{Response, StatusCode},
    router::Router,
};

#[derive(Default)]
struct TestState {
    value: usize,
    observed_value: Option<usize>,
}

fn mutate_state(_: &Request<'_>, state: &mut TestState) -> Response {
    state.value += 1;
    Response::text_plain(StatusCode::Ok, "")
}

fn observe_state(_: &Request<'_>, state: &mut TestState) -> Response {
    state.observed_value = Some(state.value);
    Response::text_plain(StatusCode::Ok, "")
}

fn parse_ok(data: &[u8]) -> Request<'_> {
    Request::parse(data).expect("request should parse")
}

#[test]
fn router_passes_mutated_state_to_a_later_handler() {
    let mut router = Router::new();
    router.post("/mutate", mutate_state);
    router.get("/observe", observe_state);
    let mut state = TestState::default();

    let mutate_request =
        parse_ok(b"POST /mutate HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n");
    router.handle_request(&mutate_request, &mut state);

    let observe_request = parse_ok(b"GET /observe HTTP/1.1\r\nHost: localhost\r\n\r\n");
    router.handle_request(&observe_request, &mut state);

    assert_eq!(state.observed_value, Some(1));
}

#[test]
fn handler_response_outlives_request_buffer_and_state() {
    struct HeaderState {
        value: String,
    }

    fn respond(request: &Request<'_>, state: &mut HeaderState) -> Response {
        let mut response = Response::text_plain(StatusCode::Ok, request.path());
        response
            .add_header("X-Request-Path", request.path())
            .expect("request path should be a valid header value");
        response
            .add_header("X-State", &state.value)
            .expect("state should be a valid header value");
        response
    }

    let response = {
        let bytes = String::from("GET /owned HTTP/1.1\r\nHost: localhost\r\n\r\n");
        let request = parse_ok(bytes.as_bytes());
        let mut state = HeaderState {
            value: String::from("ready"),
        };
        let mut router = Router::new();
        router.get("/owned", respond);
        router.handle_request(&request, &mut state)
    };
    let serialized = String::from_utf8(response.to_bytes()).unwrap();

    assert!(serialized.contains("X-Request-Path: /owned\r\n"));
    assert!(serialized.contains("X-State: ready\r\n"));
    assert!(serialized.contains("Content-Length: 6\r\n"));
    assert!(serialized.ends_with("\r\n\r\n/owned"));
}
