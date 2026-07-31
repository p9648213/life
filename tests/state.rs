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

fn mutate_state<'req>(_: &'req Request<'_>, state: &mut TestState) -> Response<'req> {
    state.value += 1;
    Response::text_plain(StatusCode::Ok, "")
}

fn observe_state<'req>(_: &'req Request<'_>, state: &mut TestState) -> Response<'req> {
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
