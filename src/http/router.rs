use std::{collections::HashMap, path::PathBuf};

use crate::http::{
    request::{HttpMethod, Request},
    response::{Response, StatusCode},
    static_files::serve_static,
};

type Handler<T> = fn(&Request, &mut T) -> Response;

pub struct Router<T> {
    get_routes: HashMap<String, Handler<T>>,
    post_routes: HashMap<String, Handler<T>>,
    static_prefix: Option<String>,
    static_dir: Option<PathBuf>,
}

impl<T> Default for Router<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Router<T> {
    pub fn new() -> Self {
        Self {
            get_routes: HashMap::new(),
            post_routes: HashMap::new(),
            static_prefix: None,
            static_dir: None,
        }
    }

    fn get_routes(&self) -> &HashMap<String, Handler<T>> {
        &self.get_routes
    }

    fn post_routes(&self) -> &HashMap<String, Handler<T>> {
        &self.post_routes
    }

    pub fn get(&mut self, name: &str, handle: Handler<T>) -> &mut Self {
        self.get_routes.insert(name.to_string(), handle);
        self
    }

    pub fn post(&mut self, name: &str, handle: Handler<T>) -> &mut Self {
        self.post_routes.insert(name.to_string(), handle);
        self
    }

    pub fn static_files(&mut self, static_prefix: &str, static_dir: &str) {
        self.static_prefix = Some(static_prefix.to_string());
        self.static_dir = Some(PathBuf::from(static_dir));
    }

    pub fn handle_request(&self, request: &Request, state: &mut T) -> Response {
        if let Some(static_prefix) = &self.static_prefix
            && let Some(static_dir) = &self.static_dir
            && let Some(asset_part) = str::strip_prefix(request.path(), static_prefix)
        {
            match request.method() {
                HttpMethod::Get => {
                    return serve_static(static_dir, asset_part);
                }
                HttpMethod::Post => return Response::new(StatusCode::MethodNotAllowed, vec![]),
            }
        }
        match request.method() {
            HttpMethod::Get => {
                let routes = self.get_routes();
                let path = request.path();
                if let Some(handler) = routes.get(path) {
                    handler(request, state)
                } else {
                    Response::html(StatusCode::NotFound, "<h1>404 Not Found</h1>")
                }
            }
            HttpMethod::Post => {
                let routes = self.post_routes();
                let path = request.path();
                if let Some(handler) = routes.get(path) {
                    handler(request, state)
                } else {
                    Response::html(StatusCode::NotFound, "<h1>404 Not Found</h1>")
                }
            }
        }
    }
}
