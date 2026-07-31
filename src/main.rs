use life::{
    constant::STORAGE_URL,
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    server::Server,
    state::State,
    storage::store::Store,
};

fn create_resourse<'buf, 'req>(request: &'req Request<'buf>, _state: &mut State) -> Response<'req> {
    if let Ok([_r_name]) = request.extract_form(["create_r_name"]) {
        // TODO persistence
        Response::see_other("/resources")
            .unwrap_or_else(|err| Response::html(StatusCode::InternalServerError, &err.to_string()))
    } else {
        Response::text_plain(StatusCode::InternalServerError, "Error Parsing Form")
    }
}

fn delete_resourse<'buf, 'req>(request: &'req Request<'buf>, _state: &mut State) -> Response<'req> {
    if let Ok([_r_name]) = request.extract_form(["delete_r_name"]) {
        // TODO persistence
        Response::see_other("/resources")
            .unwrap_or_else(|err| Response::html(StatusCode::InternalServerError, &err.to_string()))
    } else {
        Response::html(StatusCode::BadRequest, "Id Not Found")
    }
}

fn list_resourse<'buf, 'req>(_request: &'req Request<'buf>, state: &mut State) -> Response<'req> {
    let store = &state.store;
    store.execute("LOAD resource");
    let html = String::new();
    Response::html(StatusCode::Ok, &html)
}

fn main() -> std::io::Result<()> {
    let store = Store::connect(STORAGE_URL).map_err(std::io::Error::other)?;
    let state = State { store };
    let mut server = Server::new(state);
    server.routes.post("/resources/create", create_resourse);
    server.routes.post("/resources/delete", delete_resourse);
    server.routes.get("/resources", list_resourse);
    server.run("127.0.0.1:8080")
}
