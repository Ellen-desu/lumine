use lumine::{
    IntoResponse, Lumine, Request, Result,
    http::{Method, StatusCode},
};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
struct User {
    name: String,
    age: u8,
}

fn main() -> Result<()> {
    let app = Lumine::builder().route("/users", create_user).build();
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let rx = app.serve(listener);
    while let Ok(client) = rx.recv() {
        println!("{client:#?}");
    }

    Ok(())
}

// Simulation route for creating a user
fn create_user(req: Request) -> impl IntoResponse {
    if !(req.method() == Method::POST) {
        return StatusCode::METHOD_NOT_ALLOWED;
    }

    // Try to send a request: curl -d '{"name": "John Doe", "age": 16 }' 127.0.0.1:8080/users
    let user: User = serde_json::from_slice(req.body()).unwrap();

    println!("{user:#?}");

    StatusCode::CREATED
}
