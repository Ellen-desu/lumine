use lumine::{
    Body, Middleware, Next, Params, Path, Request, Response, Result, http,
    routing::route_service::RouteService,
};
use std::sync::{Arc, Mutex};

struct MockRoute;

impl RouteService for MockRoute {
    fn matches(&self, _: &Path) -> Option<Params> {
        None
    }

    fn is_duplicated(&self, _: &Path) -> bool {
        false
    }

    fn middlewares(&self) -> &[Box<dyn Middleware>] {
        &[]
    }

    fn route_middleware_first(&self) -> bool {
        false
    }

    fn call(&self, _: Request) -> Result<Response> {
        Ok(http::Response::new(Body::Empty))
    }
}

struct AppendMiddleware {
    text: &'static str,
    output: Arc<Mutex<Vec<&'static str>>>,
}

impl Middleware for AppendMiddleware {
    fn handle(&self, request: Request, next: Next) -> Result<Response> {
        self.output.lock().unwrap().push(self.text);

        next.run(request)
    }
}

#[test]
fn calling_middleware_and_ordering() {
    let output = Arc::new(Mutex::new(Vec::new()));

    let mw1 = AppendMiddleware {
        text: "mw1",
        output: output.clone(),
    };

    let mw2 = AppendMiddleware {
        text: "mw2",
        output: output.clone(),
    };

    let middlewares: &[&dyn Middleware] = &[&mw1, &mw2];

    let route = MockRoute;

    let next = Next::new(middlewares, &route);
    let request = http::Request::new(Vec::new());

    next.run(request).unwrap();

    let result = output.lock().unwrap();

    assert_eq!(&*result, &["mw1", "mw2"]);
}
