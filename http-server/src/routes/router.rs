use crate::client;
use crate::http::method::Method;
use crate::http::request::Request;
use crate::http::response::Response;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

pub struct Router {
    registered_routes: HashMap<&'static str, Vec<(Method, fn(&Request, &mut Response))>>
}

impl Router {
    pub fn new() -> Self {
        Router {
            registered_routes: HashMap::new()
        }
    }

    pub fn add(&mut self, endpoint: &'static str , method: Method, handler: fn(&Request, &mut Response)) {
        let route = self.registered_routes.get_mut(endpoint);
        if route.is_none() {
            route.
        }
        self.registered_routes.insert(endpoint, )
    }

    pub fn handle(&self, request: &Request) -> Response {
        let mut response = Response::new();
        let route = request.endpoint();

        let handlers = &self.registered_routes[route];
        handlers
            .iter()
            .filter(|x| x.0 == request.method())
            .for_each(|x| x.1(request, &mut response));
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_routes() {
        let router = Router::new();

    }
}
