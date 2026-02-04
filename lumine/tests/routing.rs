use lumine::Lumine;

#[test]
fn routing() {
    let callback = |_| ();

    Lumine::builder()
        .route("/", callback)
        .route("/users", callback)
        .route("/users/:id", callback)
        .route("/users/:id/email", callback)
        .route("/posts", callback)
        .route("/posts/:id", callback)
        .route("/posts/:id/author", callback);
}

#[test]
#[should_panic(expected = "Conflicting route")]
fn overlapping_1() {
    let callback = |_| ();

    Lumine::builder()
        .route("/users/:id", callback)
        .route("/users/:id", callback);
}

#[test]
#[should_panic(expected = "Conflicting route")]
fn overlapping_2() {
    let callback = |_| ();
    Lumine::builder()
        .route("/users/:id", callback)
        .route("/users/:id/", callback);
}

#[test]
#[should_panic(expected = "Path can't be empty or doesn't start with \"/\"")]
fn invalid_route() {
    Lumine::builder().route("users/:id", |_| ());
}
