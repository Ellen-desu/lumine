use bytes::Bytes;
use lumine::{internal::dispatch::dispatch_request, prelude::*};
use std::sync::Arc;

#[tokio::test]
async fn dispatch_request_normal_handler() {
    let app = Arc::new(Lumine::builder().route("/", async |_| ()).build());
    let request = http::Request::builder()
        .uri("/")
        .body(Bytes::new())
        .unwrap();
    let response = dispatch_request(request, &app).await;

    assert_eq!(response.status(), StatusCode::OK);
}
