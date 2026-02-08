use lumine::{Lumine, Params};
use pretty_assertions::assert_eq;
use std::net::TcpListener;

#[cfg(test)]
mod overlapping {
    use super::*;

    #[test]
    #[should_panic(expected = "Conflicting route")]
    fn test_overlapping_1() {
        let callback = |_| "Ok";
        Lumine::builder()
            .route("/users/:id", callback)
            .route("/users/:id", callback);
    }

    #[test]
    #[should_panic(expected = "Conflicting route")]
    fn test_overlapping_2() {
        let callback = |_| "Ok";
        Lumine::builder()
            .route("/users/:id", callback)
            .route("/users/:id/", callback);
    }
}

#[cfg(test)]
mod invalid_routing {
    use super::*;

    #[test]
    #[should_panic(expected = "Path can't be empty or doesn't start with \"/\"")]
    fn test_invalid_routing_1() {
        Lumine::builder().route("", |_| ());
    }

    #[test]
    #[should_panic(expected = "Path can't be empty or doesn't start with \"/\"")]
    fn test_invalid_routing_2() {
        Lumine::builder().route("users/:id", |_| ());
    }
}

#[test]
fn test_static_route_matching() {
    let app = Lumine::builder()
        .route("/users", |_| "users list")
        .route("/posts", |_| "posts list")
        .build();

    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Test static routes
    let response = ureq::get("http://127.0.0.1:8081/users").call();
    assert!(response.is_ok());

    let response = ureq::get("http://127.0.0.1:8081/posts").call();
    assert!(response.is_ok());

    // Not found
    let response = ureq::get("http://127.0.0.1:8081/comments").call();
    assert!(matches!(response, Err(ureq::Error::StatusCode(404))));
}

#[test]
fn test_dynamic_route_matching_single_param() {
    let app = Lumine::builder()
        .route("/users/:id", |req| {
            let params = Params::from_request(&req).unwrap();
            let id = params.get("id").unwrap();
            format!("User {}", id)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:8082").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Should match
    let response = ureq::get("http://127.0.0.1:8082/users/123").call();
    assert!(response.is_ok());

    // Should NOT match (different segment count)
    let response = ureq::get("http://127.0.0.1:8082/users").call();
    assert!(matches!(response, Err(ureq::Error::StatusCode(404))));

    // Should NOT match (extra segments)
    let response = ureq::get("http://127.0.0.1:8082/users/123/posts").call();
    assert!(matches!(response, Err(ureq::Error::StatusCode(404))));
}

#[test]
fn test_dynamic_route_multiple_params() {
    let app = Lumine::builder()
        .route("/users/:userId/posts/:postId", |req| {
            let params = Params::from_request(&req).unwrap();
            let user_id = params.get("userId").unwrap();
            let post_id = params.get("postId").unwrap();
            format!("User {} Post {}", user_id, post_id)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:8083").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Should match dan extract both params
    let response = ureq::get("http://127.0.0.1:8083/users/42/posts/99").call();
    assert!(response.is_ok());

    // Should NOT match (missing one param)
    let response = ureq::get("http://127.0.0.1:8083/users/42/posts").call();
    assert!(matches!(response, Err(ureq::Error::StatusCode(404))));
}

#[test]
fn test_route_priority_static_vs_dynamic() {
    let app = Lumine::builder()
        .route("/users/me", |_| "current user")
        .route("/users/:id", |_| "specific user")
        .build();

    let listener = TcpListener::bind("127.0.0.1:8084").unwrap();
    let _rx = app.serve(listener).unwrap();

    let mut response = ureq::get("http://127.0.0.1:8084/users/me").call().unwrap();
    let body = response.body_mut().read_to_string().unwrap();

    assert_eq!(body, "current user");
}

#[test]
fn test_trailing_slash_normalization() {
    let app = Lumine::builder().route("/users", |_| "users").build();

    let listener = TcpListener::bind("127.0.0.1:8085").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Both should match
    let response1 = ureq::get("http://127.0.0.1:8085/users").call();
    assert!(response1.is_ok());

    let response2 = ureq::get("http://127.0.0.1:8085/users/").call();
    assert!(response2.is_ok());
}

#[test]
fn test_deep_nested_routes() {
    let app = Lumine::builder()
        .route(
            "/api/v1/users/:userId/posts/:postId/comments/:commentId",
            |req| {
                let params = Params::from_request(&req).unwrap();
                let comment_id = params.get("commentId").unwrap();
                format!("Comment {}", comment_id)
            },
        )
        .build();

    let listener = TcpListener::bind("127.0.0.1:8086").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:8086/api/v1/users/1/posts/2/comments/3").call();
    assert!(response.is_ok());
}

#[test]
fn test_root_path() {
    let app = Lumine::builder().route("/", |_| "home").build();

    let listener = TcpListener::bind("127.0.0.1:8087").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:8087/").call();
    assert!(response.is_ok());

    // Should not match non-root
    let response = ureq::get("http://127.0.0.1:8087/anything").call();
    assert!(matches!(response, Err(ureq::Error::StatusCode(404))));
}
