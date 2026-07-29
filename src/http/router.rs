use std::collections::HashMap;

use crate::http::{
    request::{HttpMethod, Request},
    response::{Response, StatusCode},
};

type Handler<T> = for<'req, 'buf> fn(&'req Request<'buf>, &mut T) -> Response<'req>;

pub struct Router<'route, T> {
    get_routes: HashMap<&'route str, Handler<T>>,
    post_routes: HashMap<&'route str, Handler<T>>,
}

impl<'route, T> Default for Router<'route, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'route, T> Router<'route, T> {
    pub fn new() -> Self {
        Self {
            get_routes: HashMap::new(),
            post_routes: HashMap::new(),
        }
    }

    fn get_routes(&self) -> &HashMap<&'route str, Handler<T>> {
        &self.get_routes
    }

    fn post_routes(&self) -> &HashMap<&'route str, Handler<T>> {
        &self.post_routes
    }

    pub fn get(&mut self, name: &'route str, handle: Handler<T>) -> &mut Self {
        self.get_routes.insert(name, handle);
        self
    }

    pub fn post(&mut self, name: &'route str, handle: Handler<T>) -> &mut Self {
        self.post_routes.insert(name, handle);
        self
    }

    pub fn handle_request<'req, 'buf>(
        &self,
        request: &'req Request<'buf>,
        state: &mut T,
    ) -> Response<'req> {
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
