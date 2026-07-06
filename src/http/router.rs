use std::collections::HashMap;

use crate::http::{
    request::{HttpMethod, Request},
    response::{Response, StatusCode},
};

type Handler = for<'req, 'buf> fn(&'req Request<'buf>) -> Response<'req>;

pub struct Router<'route> {
    get_routes: HashMap<&'route str, Handler>,
    post_routes: HashMap<&'route str, Handler>,
}

impl<'route> Default for Router<'route> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'route> Router<'route> {
    pub fn new() -> Self {
        Self {
            get_routes: HashMap::new(),
            post_routes: HashMap::new(),
        }
    }

    fn get_routes(&self) -> &HashMap<&'route str, Handler> {
        &self.get_routes
    }

    fn post_routes(&self) -> &HashMap<&'route str, Handler> {
        &self.post_routes
    }

    pub fn get(&mut self, name: &'route str, handle: Handler) -> &mut Self {
        self.get_routes.insert(name, handle);
        self
    }

    pub fn post(&mut self, name: &'route str, handle: Handler) -> &mut Self {
        self.post_routes.insert(name, handle);
        self
    }

    pub fn handle_request<'req, 'buf>(&self, request: &'req Request<'buf>) -> Response<'req> {
        match request.method() {
            HttpMethod::Get => {
                let routes = self.get_routes();
                let path = request.path();

                if let Some(handler) = routes.get(path) {
                    handler(request)
                } else {
                    Response::html(StatusCode::NotFound, "<h1>404 Not Found</h1>")
                }
            }
            HttpMethod::Post => {
                let routes = self.post_routes();
                let path = request.path();

                if let Some(handler) = routes.get(path) {
                    handler(request)
                } else {
                    Response::html(StatusCode::NotFound, "<h1>404 Not Found</h1>")
                }
            }
        }
    }
}
