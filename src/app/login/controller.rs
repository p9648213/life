use crate::{
    app::login::model::User,
    constant::USER_COLLECTION,
    http::{
        cookie::Cookie,
        request::{HttpMethod, Request},
        response::{Response, StatusCode},
    },
    state::State,
    storage::util::{create_session, hash_password},
    templates,
};

pub fn login(request: &Request, state: &mut State) -> Response {
    match request.method() {
        HttpMethod::Get => {
            let mut out = String::new();
            templates::render_auth_login(&mut out);
            let res = hash_password("aa");
            let res2 = hash_password("bB");
            println!("{}", res == res2);
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
                        Some(user) => {
                            let password = &user.password;
                            Response::text_plain(StatusCode::Ok, "Ok")
                        }
                        None => Response::text_plain(StatusCode::NotFound, "User not found"),
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
                            let item = User::new(input_username, input_password, session.clone());
                            match resource_collection.insert_one(item) {
                                Ok(_) => match Response::see_other("/") {
                                    Ok(mut response) => {
                                        let mut cookie = Cookie::new();
                                        match cookie.set_name_value("session", &session) {
                                            Ok(_) => {
                                                match response.add_header("Cookie", &cookie.build())
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
