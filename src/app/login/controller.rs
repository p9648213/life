use crate::{
    app::login::model::User,
    constant::{SET_COOKIE, USER_COLLECTION},
    http::{
        cookie::Cookie,
        request::{HttpMethod, Request},
        response::{Response, StatusCode},
    },
    state::State,
    templates,
    util::{create_session, hash_password},
};

pub fn login(request: &Request, state: &mut State) -> Response {
    if request.get_cookie_value("session").is_some() {
        match Response::see_other("/resources") {
            Ok(response) => return response,
            Err(err) => {
                return Response::text_plain(StatusCode::InternalServerError, &err.to_string());
            }
        }
    }
    match request.method() {
        HttpMethod::Get => {
            let mut out = String::new();
            templates::render_auth_login(&mut out);
            Response::html(StatusCode::Ok, &out)
        }
        HttpMethod::Post => match request.extract_form(["username", "password"]) {
            Ok([input_username, input_password]) => {
                let store = &state.store;
                let mut user_collection = match store.collection::<User>(USER_COLLECTION) {
                    Ok(user_collection) => user_collection,
                    Err(err) => {
                        return Response::text_plain(
                            StatusCode::InternalServerError,
                            &err.to_string(),
                        );
                    }
                };
                match user_collection.list() {
                    Ok(users) => match users.iter().find(|u| u.username == input_username) {
                        Some(user) => {
                            let hash_input_password = hash_password(&input_password);
                            if user.password != hash_input_password {
                                Response::text_plain(StatusCode::BadRequest, "Password incorrect")
                            } else {
                                let new_session = create_session();
                                state.session.insert(new_session.clone(), user.id);
                                match Response::see_other("/resources") {
                                    Ok(mut response) => {
                                        let mut cookie = Cookie::new();
                                        match cookie.set_name_value("session", &new_session) {
                                            Ok(_) => {
                                                match response
                                                    .add_header(SET_COOKIE, &cookie.build())
                                                {
                                                    Ok(_) => response,
                                                    Err(err) => Response::text_plain(
                                                        StatusCode::InternalServerError,
                                                        &err.to_string(),
                                                    ),
                                                }
                                            }
                                            Err(err) => Response::text_plain(
                                                StatusCode::InternalServerError,
                                                &err.to_string(),
                                            ),
                                        }
                                    }
                                    Err(err) => Response::text_plain(
                                        StatusCode::InternalServerError,
                                        &err.to_string(),
                                    ),
                                }
                            }
                        }
                        None => Response::text_plain(StatusCode::NotFound, "Username not found"),
                    },
                    Err(err) => {
                        Response::text_plain(StatusCode::InternalServerError, &err.to_string())
                    }
                }
            }
            Err(err) => Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
        },
    }
}

pub fn register(request: &Request, state: &mut State) -> Response {
    if request.get_cookie_value("session").is_some() {
        match Response::see_other("/resources") {
            Ok(response) => return response,
            Err(err) => {
                return Response::text_plain(StatusCode::InternalServerError, &err.to_string());
            }
        }
    }
    match request.method() {
        HttpMethod::Get => {
            let mut out = String::new();
            templates::render_auth_register(&mut out);
            Response::html(StatusCode::Ok, &out)
        }
        HttpMethod::Post => match request.extract_form(["username", "password"]) {
            Ok([input_username, input_password]) => {
                let store = &state.store;
                let mut resource_collection = match store.collection::<User>(USER_COLLECTION) {
                    Ok(resource_collection) => resource_collection,
                    Err(err) => {
                        return Response::text_plain(
                            StatusCode::InternalServerError,
                            &err.to_string(),
                        );
                    }
                };
                match resource_collection.list() {
                    Ok(users) => match users.iter().find(|u| u.username == input_username) {
                        Some(_) => Response::text_plain(StatusCode::BadRequest, "Username exist"),
                        None => {
                            let session = create_session();
                            let hash_password = hash_password(&input_password);
                            let item = User::new(input_username, hash_password, session.clone());
                            match resource_collection.insert_one(item) {
                                Ok(_) => match Response::see_other("/resources") {
                                    Ok(mut response) => {
                                        let mut cookie = Cookie::new();
                                        match cookie.set_name_value("session", &session) {
                                            Ok(_) => {
                                                match response
                                                    .add_header(SET_COOKIE, &cookie.build())
                                                {
                                                    Ok(_) => response,
                                                    Err(err) => Response::text_plain(
                                                        StatusCode::InternalServerError,
                                                        &err.to_string(),
                                                    ),
                                                }
                                            }
                                            Err(err) => Response::text_plain(
                                                StatusCode::InternalServerError,
                                                &err.to_string(),
                                            ),
                                        }
                                    }
                                    Err(err) => Response::text_plain(
                                        StatusCode::InternalServerError,
                                        &err.to_string(),
                                    ),
                                },
                                Err(err) => Response::text_plain(
                                    StatusCode::InternalServerError,
                                    &err.to_string(),
                                ),
                            }
                        }
                    },
                    Err(err) => {
                        Response::text_plain(StatusCode::InternalServerError, &err.to_string())
                    }
                }
            }
            Err(err) => Response::text_plain(StatusCode::InternalServerError, &err.to_string()),
        },
    }
}
