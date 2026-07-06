use std::collections::HashMap;

use crate::http::{request::Request, response::Response};

type Handler = for<'a> fn(&'a Request) -> Response<'a>;

pub struct Router<'a> {
    get_routes: HashMap<&'a str, Handler>,
    post_routes: HashMap<&'a str, Handler>,
}

impl<'a> Default for Router<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Router<'a> {
    pub fn new() -> Self {
        Self {
            get_routes: HashMap::new(),
            post_routes: HashMap::new(),
        }
    }

    pub fn get_routes(&self) -> &HashMap<&'a str, Handler> {
        &self.get_routes
    }

    pub fn post_routes(&self) -> &HashMap<&'a str, Handler> {
        &self.post_routes
    }

    pub fn get(&mut self, name: &'a str, handle: Handler) -> &mut Self {
        self.get_routes.insert(name, handle);
        self
    }

    pub fn post(&mut self, name: &'a str, handle: Handler) -> &mut Self {
        self.post_routes.insert(name, handle);
        self
    }
}
