use bytes::Bytes;
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
    let request = http::Request::new(Bytes::new());

    next.run(request).await;

    let result = output.lock().unwrap();

    assert_eq!(&*result, &["mw1", "mw2"]);
}

struct ShortCircuitMiddleware;

#[async_trait::async_trait]
impl Middleware for ShortCircuitMiddleware {
    async fn handle(&self, _request: Request, _next: Next) -> Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

#[tokio::test]
async fn middleware_short_circuit() {
    let route = Route::new(vec![], vec![], async |_| "ok");

    let output = Arc::new(Mutex::new(Vec::new()));

    let mw1 = Arc::new(ShortCircuitMiddleware);
    let mw2 = Arc::new(AppendMiddleware {
        text: "mw2",
        output: output.clone(),
    });

    let middlewares: Vec<Arc<dyn Middleware>> = vec![mw1, mw2];

    let next = Next::new(middlewares, Arc::new(route));
    let request = http::Request::new(Bytes::new());

    let response = next.run(request).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // mw2 should not have been called
    let result = output.lock().unwrap();
    assert!(result.is_empty());
}

#[tokio::test]
async fn middleware_run_before_global() {
    use lumine::internal::dispatch::dispatch_request;

    let output = Arc::new(Mutex::new(Vec::new()));

    let app = Arc::new(
        Lumine::builder()
            .middleware(AppendMiddleware {
                text: "global",
                output: output.clone(),
            })
            .route_with(
                "/",
                async |_| "ok",
                |r| {
                    r.middleware(AppendMiddleware {
                        text: "route",
                        output: output.clone(),
                    })
                    .run_before_global()
                },
            )
            .build(),
    );

    let request = http::Request::builder()
        .uri("/")
        .body(Bytes::new())
        .unwrap();
    dispatch_request(request, &app).await;

    let result = output.lock().unwrap();
    assert_eq!(&*result, &["route", "global"]);
}
