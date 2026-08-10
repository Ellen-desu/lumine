use lumine::{prelude::*, routing::route::Route};
use std::sync::{Arc, Mutex};

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
    let route = Route::new(vec![], vec![], async |_| ());

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

    let next = Next::new(middlewares, Arc::new(route));
    let request = http::Request::new(Vec::new());

    next.run(request).await;

    let result = output.lock().unwrap();

    assert_eq!(&*result, &["mw1", "mw2"]);
}
