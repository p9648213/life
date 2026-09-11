use crate::{
    app::resource::model::Resource,
    constant::RESOURCE_COLLECTION,
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    state::State,
    storage::error::StoreError,
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
            let mut resource_collection = match store.collection::<Resource>(RESOURCE_COLLECTION) {
                Ok(resource_collection) => resource_collection,
                Err(err) => {
                    return Response::text_plain(StatusCode::InternalServerError, &err.to_string());
                }
            };
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
        let mut resource_collection = match store.collection::<Resource>(RESOURCE_COLLECTION) {
            Ok(resource_collection) => resource_collection,
            Err(err) => {
                return Response::text_plain(StatusCode::InternalServerError, &err.to_string());
            }
        };
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

pub fn update_resource<'buf, 'req>(
    request: &'req Request<'buf>,
    state: &mut State,
) -> Response<'req> {
    if let Ok([r_id, r_name, r_number]) =
        request.extract_form(["update_r_id", "update_r_name", "update_r_number"])
    {
        let store = &state.store;
        let mut resource_collection = match store.collection::<Resource>(RESOURCE_COLLECTION) {
            Ok(resource_collection) => resource_collection,
            Err(err) => {
                return Response::text_plain(StatusCode::InternalServerError, &err.to_string());
            }
        };
        let Ok(r_id) = r_id.parse::<u32>() else {
            return Response::html(StatusCode::BadRequest, "Error parsing id");
        };
        let Ok(r_number) = r_number.parse::<u32>() else {
            return Response::html(StatusCode::BadRequest, "Error parsing number");
        };
        let update_resource = Resource {
            id: r_id,
            name: r_name,
            number: r_number,
        };
        match resource_collection.update_one(r_id, update_resource) {
            Ok(_) => Response::see_other("/resources").unwrap_or_else(|err| {
                Response::html(StatusCode::InternalServerError, &err.to_string())
            }),
            Err(err) => Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
        }
    } else {
        Response::html(StatusCode::BadRequest, "Missing fields")
    }
}

pub fn list_resourse<'buf, 'req>(
    request: &'req Request<'buf>,
    state: &mut State,
) -> Response<'req> {
    let id = match request.query().get("id") {
        Some(value) => match value.parse::<u32>() {
            Ok(id) if id > 0 => Some(id),
            _ => return Response::text_plain(StatusCode::BadRequest, "Invalid resource ID"),
        },
        None => None,
    };
    let store = &state.store;
    let mut resource_collection = match store.collection::<Resource>(RESOURCE_COLLECTION) {
        Ok(resource_collection) => resource_collection,
        Err(err) => return Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
    };
    let resources = match id {
        Some(id) => resource_collection
            .find_one(id)
            .map(|resource| vec![resource]),
        None => resource_collection.list(),
    };
    match resources {
        Ok(resources) => {
            let total = resources.len().to_string();
            let mut html = String::new();
            let mut cards = String::new();
            for resource in resources {
                templates::render_resource_card(
                    &mut cards,
                    templates::ResourceCardView {
                        id: &resource.id.to_string(),
                        name: &resource.name,
                        number: &resource.number.to_string(),
                    },
                );
            }
            if cards.is_empty() {
                cards.push_str(
                    "<p class=\"empty-state\">No resources yet. Create one using the form.</p>",
                );
            }
            let view = templates::ResourceResourceView {
                list_resource: &cards,
                total: &total,
            };
            templates::render_resource_resource(&mut html, view);
            Response::html(StatusCode::Ok, &html)
        }
        Err(StoreError::StorageIndexIdNotFound) => {
            Response::text_plain(StatusCode::NotFound, "Resource not found")
        }
        Err(err) => Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
    }
}
