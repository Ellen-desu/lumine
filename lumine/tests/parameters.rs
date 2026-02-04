use std::net::TcpListener;

use lumine::{IntoResponse, Lumine, Params, Query, Request, Result};

#[test]
fn parameters() -> Result<()> {
    let app = Lumine::builder()
        .route("/users/:userid/orders/:orderid", callback)
        .build();
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    app.serve(listener)?;

    ureq::get("http://127.0.0.1:8080/users/12/orders/10?status=active&limit=10&limit=20&")
        .call()
        .unwrap();

    Ok(())
}

fn callback(req: Request) -> impl IntoResponse {
    // Params and Query structs will always available even if the route doesn't have or accept parameters
    let path_params = Params::from_request(&req).unwrap();
    let query_params = Query::from_request(&req).unwrap();

    assert_eq!(path_params.get("userid"), Some(&"12".to_string()));
    assert_eq!(path_params.get("orderid"), Some(&"10".to_string()));

    assert_eq!(query_params.get("status"), Some(&vec!["active".into()]));
    assert_eq!(
        query_params.get("limit"),
        Some(&vec!["10".into(), "20".into()]) // Supports multiple query parameters
    );
}
