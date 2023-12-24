use crate::request::Request;
use crate::response::Response;
use std::collections::HashMap;

type RouteHandler = Box<dyn Fn(&Request) -> Response + Send + Sync>;

pub struct Router {
    routes: HashMap<String, RouteHandler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, method: &str, path: &str, handler: F)
    where
        F: Fn(&Request) -> Response + 'static + Send + Sync,
    {
        self.routes
            .insert(format!("{} {}", method, path), Box::new(handler));
    }

    pub fn route(&self, request: &Request) -> Option<&RouteHandler> {
        self.routes
            .get(&format!("{} {}", request.method, request.uri))
    }
}
