use crate::request::Request;
use crate::response::Response;

pub type MiddlewareFn = Box<dyn Fn(&Request, &Response) -> Response + Send + Sync>;

pub struct Middleware {
    middlewares: Vec<MiddlewareFn>,
}

impl Middleware {
    pub fn new() -> Self {
        Middleware {
            middlewares: Vec::new(),
        }
    }

    pub fn add(&mut self, middleware: MiddlewareFn) {
        self.middlewares.push(middleware);
    }

    pub fn run(&self, req: &Request, res: &Response) -> Response {
        let mut current_res = res.clone();
        for middleware in &self.middlewares {
            current_res = middleware(req, &current_res);
        }
        current_res
    }
}