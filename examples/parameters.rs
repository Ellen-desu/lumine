use std::net::TcpListener;

use lumine::{IntoResponse, Lumine, Params, Query, Request, Result};

fn main() -> Result<()> {
    let app = Lumine::builder().route("/:user_id", user).build();
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // You have to handle client error and do a while loop inside it. Otherwise, the event loop never starts
    if let Ok(rx) = app.serve(listener) {
        while let Ok(error) = rx.recv() {
            eprintln!("Client error: {error}");
        }
    }

    Ok(())
}

fn user(req: Request) -> impl IntoResponse {
    let params = Params::from_request(&req).unwrap();
    let _user_id = params.get("user_id");

    let query = Query::from_request(&req).unwrap();
    let _search = query.get("search");

    println!("{params:#?}");
    println!("{query:#?}");
}
