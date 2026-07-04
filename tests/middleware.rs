use lumine::{
    prelude::*,
    routing::{route_service::RouteService, segment::Segment},
};
use std::sync::{Arc, Mutex};

struct MockRoute;

#[async_trait::async_trait]
impl RouteService for MockRoute {
    fn matches(&self, _: &[&str]) -> Option<Params> {
        None
    }

    fn is_duplicated(&self, _: &[Segment]) -> bool {
        false
    }

    fn middlewares(&self) -> &[Arc<dyn Middleware>] {
        &[]
    }

    fn run_before_global(&self) -> bool {
        false
    }

    async fn call(&self, _: Request) -> Response {
        http::Response::new(Body::Empty)
    }
}

struct AppendMiddleware {
    text: &'static str,
    output: Arc<Mutex<Vec<&'static str>>>,
}

#[async_trait::async_trait]
impl Middleware for AppendMiddleware {
    async fn handle(&self, request: Request, next: Next) -> Response {
        self.output.lock().unwrap().push(self.text);

        next.run(request).await
    }
}

#[tokio::test]
async fn calling_middleware_and_ordering() {
    let output = Arc::new(Mutex::new(Vec::new()));

    let mw1 = Arc::new(AppendMiddleware {
        text: "mw1",
        output: output.clone(),
    });

    let mw2 = Arc::new(AppendMiddleware {
        text: "mw2",
        output: output.clone(),
    });

    let middlewares: Vec<Arc<dyn Middleware>> = vec![mw1, mw2];

    let next = Next::new(middlewares, Arc::new(MockRoute));
    let request = http::Request::new(Vec::new());

    next.run(request).await;

    let result = output.lock().unwrap();

    assert_eq!(&*result, &["mw1", "mw2"]);
}
