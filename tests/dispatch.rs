use lumine::{internal::dispatch::dispatch_request, prelude::*};
use std::sync::Arc;

#[tokio::test]
async fn dispatch_request_normal_handler() {
    let app = Arc::new(Lumine::builder().route("/", async |_| ()).build());
    let request = http::Request::builder().uri("/").body(Vec::new()).unwrap();
    let response = dispatch_request(request, &app).await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn dispatch_request_panic_handler() {
    let app = Arc::new(
        Lumine::builder()
            .route("/", async |_| -> Response { panic!() })
            .build(),
    );
    let request = http::Request::builder().uri("/").body(Vec::new()).unwrap();
    let response = dispatch_request(request, &app).await;

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
