use life::{
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    server::Server,
    state::State,
    templates::{ResourceView, render_resource},
};

fn create_resourse<'buf, 'req>(request: &'req Request<'buf>, state: &mut State) -> Response<'req> {
    if let Ok([r_name]) = request.extract_form(["create_r_name"]) {
        state.resources.push(r_name);
        let resources = state.resources.join(", ");
        let view = ResourceView {
            list_resource: &resources,
        };
        let mut html = String::new();
        render_resource(&mut html, view);
        Response::html(StatusCode::Ok, &html)
    } else {
        Response::text_plain(StatusCode::InternalServerError, "Error Parsing Form")
    }
}

fn delete_resourse<'buf, 'req>(request: &'req Request<'buf>, state: &mut State) -> Response<'req> {
    if let Ok([r_name]) = request.extract_form(["delete_r_name"]) {
        state.resources.retain(|r| *r != r_name);
        let resources = state.resources.join(", ");
        let view = ResourceView {
            list_resource: &resources,
        };
        let mut html = String::new();
        render_resource(&mut html, view);
        Response::html(StatusCode::Ok, &html)
    } else {
        Response::html(StatusCode::BadRequest, "Id Not Found")
    }
}

fn list_resourse<'buf, 'req>(request: &'req Request<'buf>, state: &mut State) -> Response<'req> {
    let mut html = String::new();
    if let Some(r_name) = request.query().get("name").copied() {
        let resource = state
            .resources
            .iter()
            .find(|r| *r == r_name)
            .map(String::as_str)
            .unwrap_or("");
        let view = ResourceView {
            list_resource: resource,
        };
        render_resource(&mut html, view);
    } else {
        let resources = state.resources.join(",");
        let view = ResourceView {
            list_resource: &resources,
        };
        render_resource(&mut html, view);
    }
    Response::html(StatusCode::Ok, &html)
}

fn main() -> std::io::Result<()> {
    let state = State::default();
    let mut server = Server::new(state);
    server.routes.post("/resources/create", create_resourse);
    server.routes.post("/resources/delete", delete_resourse);
    server.routes.get("/resources", list_resourse);
    server.run("127.0.0.1:8080")
}
