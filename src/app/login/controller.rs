use crate::{
    http::{
        request::{HttpMethod, Request},
        response::{Response, StatusCode},
    },
    state::State,
    templates,
};

pub fn login(request: &Request, state: &mut State) -> Response {
    let method = request.method();
    match method {
        HttpMethod::Get => {
            let mut out = String::new();
            templates::render_login_login(&mut out);
            Response::html(StatusCode::Ok, &out)
        }
        HttpMethod::Post => todo!(),
    }
}
