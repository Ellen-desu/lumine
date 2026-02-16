use lumine::{IntoResponse, Lumine, Request, Result, http::StatusCode};
use std::net::TcpListener;

fn main() -> Result<()> {
    let app = Lumine::builder()
        .set_workers(4)
        .route("/", |_| "Hello, World!")
        .route("/health", health)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let rx = app.serve(listener);
    while let Ok(client) = rx.recv() {
        println!("{client:#?}");
    }

    Ok(())
}

fn health(_: Request) -> impl IntoResponse {
    (StatusCode::OK, "Good")
}
