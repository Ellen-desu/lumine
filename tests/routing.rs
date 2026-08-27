use lumine::prelude::*;

#[test]
#[should_panic(expected = "Conflicting route")]
fn overlapping_root() {
    let callback = async |_| ();
    Lumine::builder().route("/", callback).route("/", callback);
}

#[test]
#[should_panic(expected = "Conflicting route")]
fn overlapping_static() {
    let callback = async |_| ();
    Lumine::builder()
        .route("/users", callback)
        .route("/users", callback);
}

#[test]
#[should_panic(expected = "Conflicting route")]
fn overlapping_unique_params() {
    let callback = async |_| ();
    Lumine::builder()
        .route("/users/:id", callback)
        .route("/users/:id", callback);
}

#[test]
#[should_panic(expected = "Conflicting route")]
fn overlapping_dynamic_params() {
    let callback = async |_| ();
    Lumine::builder()
        .route("/users/:id", callback)
        .route("/users/:name", callback);
}

#[test]
#[should_panic(expected = "Conflicting route")]
fn overlapping_wildcard() {
    let callback = async |_| ();
    Lumine::builder()
        .route("/users/:id", callback)
        .route("/users/*", callback);
}

#[test]
#[should_panic(expected = "Wildcard segment must be the last segment")]
fn path_after_wildcard() {
    let callback = async |_| ();
    Lumine::builder().route("/users/*/posts", callback);
}

#[test]
#[should_panic(expected = "Conflicting route")]
fn dynamic_overlapping_with_wildcard() {
    let callback = async |_| ();
    Lumine::builder()
        .route("/users/*", callback)
        .route("/users/:id", callback);
}

#[test]
#[should_panic(expected = "Conflicting route")]
fn static_overlapping_with_wildcard() {
    let callback = async |_| ();
    Lumine::builder()
        .route("/users/posts", callback)
        .route("/users/*", callback);
}

#[test]
#[should_panic(expected = "Path parameter must be followed by a name")]
fn no_name_parameters() {
    let callback = async |_| ();
    Lumine::builder().route("/users/:", callback);
}

#[test]
#[should_panic(expected = "Path must start with a slash and not end with a slash")]
fn path_not_start_with_slash() {
    let callback = async |_| ();
    Lumine::builder().route("users/:id", callback);
}

#[test]
#[should_panic(expected = "Path must start with a slash and not end with a slash")]
fn path_end_with_slash() {
    let callback = async |_| ();
    Lumine::builder().route("/users/:id/", callback);
}

#[test]
fn static_route_matching() {
    let app = Lumine::builder()
        .route("/", async |_| ())
        .route("/api/v2/users", async |_| ())
        .route("/posts", async |_| ())
        .build();

    assert!(app.get_route("/").is_some());
    assert!(app.get_route("/api/v2/users").is_some());
    assert!(app.get_route("/posts").is_some());
}

#[test]
fn wildcard_route_matching() {
    let app = Lumine::builder().route("/assets/*", async |_| ()).build();

    let result = app.get_route("/assets/images/image.jpeg");
    assert!(result.is_some());

    match result {
        Some((_, _, rest)) => {
            assert_eq!(rest.get(), Some("images/image.jpeg"));
        }
        None => unreachable!(),
    }

    assert!(app.get_route("/assets").is_none());
}

#[test]
fn dynamic_route_matching_and_parameters_extraction() {
    let app = Lumine::builder()
        .route("/users/:userId/posts/:postId", async |_| ())
        .build();

    let result = app.get_route("/users/1/posts/2");
    assert!(result.is_some());

    match result {
        Some((_, params, _)) => {
            assert_eq!(params.get("userId"), Some("1"));
            assert_eq!(params.get("postId"), Some("2"));
        }
        None => unreachable!(),
    }
}

#[test]
fn route_priority_static_vs_dynamic() {
    let app = Lumine::builder()
        .route("/users/me", async |_| ())
        .route("/users/:id", async |_| ())
        .build();

    let result = app.get_route("/users/me");
    assert!(result.is_some());

    match result {
        Some((_, params, _)) => {
            // The result is depends of the order of the routes, it will be error if the order is reversed
            assert!(params.get("id").is_none());
        }
        None => unreachable!(),
    }
}

#[test]
fn advanced_route_extraction_multiple_params_and_wildcard() {
    let app = Lumine::builder()
        .route(
            "/orgs/:orgId/teams/:teamId/members/:memberId/*",
            async |_| (),
        )
        .build();

    let result = app.get_route("/orgs/acme/teams/backend/members/alice/permissions/write");
    assert!(result.is_some());

    match result {
        Some((_, params, remainder)) => {
            assert_eq!(params.get("orgId"), Some("acme"));
            assert_eq!(params.get("teamId"), Some("backend"));
            assert_eq!(params.get("memberId"), Some("alice"));
            assert_eq!(remainder.get(), Some("permissions/write"));
        }
        None => unreachable!(),
    }
}
