use lumine::{Lumine, http::Uri};

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
fn static_route_matching() {
    let app = Lumine::builder()
        .route("/", |_| ())
        .route("/api/v2/users", |_| ())
        .route("/posts", |_| ())
        .build();

    assert!(app.get_route_test(&Uri::from_static("/")).is_some());
    assert!(
        app.get_route_test(&Uri::from_static("/api/v2/users"))
            .is_some()
    );
    assert!(app.get_route_test(&Uri::from_static("/posts")).is_some());
}

#[test]
fn dynamic_route_matching_and_parameters_extraction() {
    let app = Lumine::builder()
        .route("/users/:userId/posts/:postId", |_| ())
        .build();

    let result = app.get_route_test(&Uri::from_static("/users/1/posts/2"));
    assert!(result.is_some());

    match result {
        Some((_, params)) => {
            assert_eq!(params.get("userId"), Some(&String::from("1")));
            assert_eq!(params.get("postId"), Some(&String::from("2")));
        }
        None => unreachable!(),
    }
}

#[test]
fn route_priority_static_vs_dynamic() {
    let app = Lumine::builder()
        .route("/users/me", |_| ())
        .route("/users/:id", |_| ())
        .build();

    let result = app.get_route_test(&Uri::from_static("/users/me"));
    assert!(result.is_some());

    match result {
        Some((_, params)) => {
            // The result is depends of the order of the routes, it will be error if the order is reversed
            assert!(params.get("id").is_none());
        }
        None => unreachable!(),
    }
}

#[test]
fn trailing_slash_normalization() {
    let app = Lumine::builder().route("/users/:id", |_| ()).build();
    assert!(app.get_route_test(&Uri::from_static("/users/1/")).is_some());
}
