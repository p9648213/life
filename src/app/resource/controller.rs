use crate::{
    app::resource::model::Resource, constant::RESOURCE_COLLECTION, http::{
        request::Request,
        response::{Response, StatusCode},
    }, state::State
};

pub fn create_resourse<'buf, 'req>(
    request: &'req Request<'buf>,
    state: &mut State,
) -> Response<'req> {
    if let Ok([r_name, r_number]) = request.extract_form(["create_r_name", "create_r_number"]) {
        let store = &state.store;
        let item = Resource {
            name: r_name,
            number: r_number.parse().unwrap_or_default(),
        };

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
    println!("list resources");
    let store = &state.store;
    let resource = store.collection::<Resource>(RESOURCE_COLLECTION);
    let new_resource = Resource {
        name: "test".to_string(),
        number: 1
    };
    resource.insert_one(new_resource);
    let html = String::new();
    Response::html(StatusCode::Ok, &html)
}
