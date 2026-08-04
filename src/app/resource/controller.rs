use crate::{
    app::resource::model::Resource, http::{
        request::Request,
        response::{Response, StatusCode},
    }, state::State
};

pub fn create_resourse<'buf, 'req>(
    request: &'req Request<'buf>,
    _state: &mut State,
) -> Response<'req> {
    if let Ok([_r_name]) = request.extract_form(["create_r_name"]) {
        // TODO persistence
        Response::see_other("/resources")
            .unwrap_or_else(|err| Response::html(StatusCode::InternalServerError, &err.to_string()))
    } else {
        Response::text_plain(StatusCode::InternalServerError, "Error Parsing Form")
    }
}

pub fn delete_resourse<'buf, 'req>(
    request: &'req Request<'buf>,
    _state: &mut State,
) -> Response<'req> {
    if let Ok([_r_name]) = request.extract_form(["delete_r_name"]) {
        // TODO persistence
        Response::see_other("/resources")
            .unwrap_or_else(|err| Response::html(StatusCode::InternalServerError, &err.to_string()))
    } else {
        Response::html(StatusCode::BadRequest, "Id Not Found")
    }
}

pub fn list_resourse<'buf, 'req>(
    _request: &'req Request<'buf>,
    state: &mut State,
) -> Response<'req> {
    let store = &state.store;
    let _ = store.list::<Resource>("resources").unwrap_or_default();
    let html = String::new();
    Response::html(StatusCode::Ok, &html)
}
