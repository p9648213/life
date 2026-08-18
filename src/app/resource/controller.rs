use crate::{
    app::resource::model::Resource,
    constant::RESOURCE_COLLECTION,
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    state::State,
    templates,
};

pub fn create_resourse<'buf, 'req>(
    request: &'req Request<'buf>,
    state: &mut State,
) -> Response<'req> {
    match request.extract_form(["create_r_name", "create_r_number"]) {
        Ok([r_name, r_number]) => {
            let store = &state.store;
            let item = Resource::new(r_name, r_number.parse().unwrap_or_default());
            let resource_collection = store.collection::<Resource>(RESOURCE_COLLECTION);
            match resource_collection.insert_one(item) {
                Ok(_) => Response::see_other("/resources").unwrap_or_else(|err| {
                    Response::html(StatusCode::InternalServerError, &err.to_string())
                }),
                Err(err) => Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
            }
        }
        Err(err) => Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
    }
}

pub fn delete_resourse<'buf, 'req>(
    request: &'req Request<'buf>,
    state: &mut State,
) -> Response<'req> {
    if let Ok([r_id]) = request.extract_form(["delete_r_id"]) {
        let store = &state.store;
        let mut resource_collection = store.collection::<Resource>(RESOURCE_COLLECTION);
        match r_id.parse::<u32>() {
            Ok(id) => match resource_collection.delete_one(id) {
                Ok(_) => Response::see_other("/resources").unwrap_or_else(|err| {
                    Response::html(StatusCode::InternalServerError, &err.to_string())
                }),
                Err(err) => Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
            },
            Err(_) => Response::text_plain(StatusCode::BadRequest, "Invalid resourse id"),
        }
    } else {
        Response::html(StatusCode::BadRequest, "Id Not Found")
    }
}

pub fn list_resourse<'buf, 'req>(
    _request: &'req Request<'buf>,
    state: &mut State,
) -> Response<'req> {
    let store = &state.store;
    let mut resource_collection = store.collection::<Resource>(RESOURCE_COLLECTION);
    let resources = resource_collection.list();
    let total = resource_collection
        .record_count()
        .unwrap_or_default()
        .to_string();
    match resources {
        Ok(resources) => {
            let mut html = String::new();
            let resources: Vec<String> = resources
                .into_iter()
                .map(|r| format!("id: {} - name: {} - number: {}", r.id, r.name, r.number))
                .collect();
            let view = templates::ResourceView {
                list_resource: &resources.join(", "),
                total: &total,
            };
            templates::render_resource(&mut html, view);
            Response::html(StatusCode::Ok, &html)
        }
        Err(err) => Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
    }
}
