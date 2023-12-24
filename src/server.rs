use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

use crate::connection::handle_connection;
use crate::router::Router;
use crate::{Request, Response};

pub struct HttpServer {
    router: Arc<Router>,
}

pub struct ServerBuilder {
    router: Router,
}

impl ServerBuilder {
    pub fn new() -> Self {
        ServerBuilder {
            router: Router::new(),
        }
    }

    pub fn get<F>(mut self, route: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + 'static + Send + Sync,
    {
        self.router.add_route("GET", route, handler);
        self
    }

    pub fn post<F>(mut self, route: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + 'static + Send + Sync,
    {
        self.router.add_route("POST", route, handler);
        self
    }

    pub fn put<F>(mut self, route: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + 'static + Send + Sync,
    {
        self.router.add_route("PUT", route, handler);
        self
    }

    pub fn delete<F>(mut self, route: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + 'static + Send + Sync,
    {
        self.router.add_route("DELETE", route, handler);
        self
    }

    pub fn build(self) -> HttpServer {
        HttpServer {
            router: Arc::new(self.router),
        }
    }
}

impl HttpServer {
    pub fn listen(&self, port: u16, on_start: impl FnOnce()) {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
        on_start();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let router = self.router.clone();
            thread::spawn(move || {
                let result = handle_connection(stream, router);
                if let Err(e) = result {
                    println!("Error: {}", e);
                }
            });
        }
    }
}
