//! # Request Handling Example
//!
//! Demonstrates how to access and parse request data.
//!
//! ## What You'll Learn
//! - Accessing request method, URI, headers
//! - Parsing JSON request body with serde_json
//! - Error handling for invalid JSON
//! - Returning appropriate status codes based on validation
//! - Real-world POST/CREATE pattern
//!
//! ## Try It
//! ```bash
//! cargo run --example request
//!
//! # Valid request
//! curl -X POST http://127.0.0.1:8080/users \
//!   -H "Content-Type: application/json" \
//!   -d '{"name":"John Doe","email":"john@example.com","age":30}'
//!
//! # Invalid JSON
//! curl -X POST http://127.0.0.1:8080/users \
//!   -H "Content-Type: application/json" \
//!   -d '{invalid json}'
//!
//! # Check request info
//! curl -v http://127.0.0.1:8080/info
//! ```

use http::StatusCode;
use lumine::{IntoResponse, Lumine, Request, Result};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

fn main() -> Result<()> {
    let app = Lumine::builder()
        // GET endpoint that shows request info
        .route("/info", request_info_handler)
        // POST endpoint that parses JSON body
        .route("/users", create_user_handler)
        // GET endpoint for demonstration
        .route("/", |_| "POST to /users with JSON body")
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("\n📍 Request Examples:");
    println!("   GET /info              → Show request details");
    println!("   POST /users            → Create new user (JSON body)");
    println!("\n💡 Examples:");
    println!("   curl http://127.0.0.1:8080/info");
    println!("   curl -X POST http://127.0.0.1:8080/users \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{{\"name\":\"John\",\"email\":\"john@example.com\",\"age\":30}}'");
    println!("\n⏹️  Press Ctrl+C to stop\n");

    app.serve(listener)
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
    age: u32,
    created_at: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    details: String,
}

// ============================================================================
// Handlers
// ============================================================================

/// Handler: Display request information
fn request_info_handler(req: Request) -> impl IntoResponse {
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let headers_count = req.headers().len();
    let body_size = req.body().len();

    let info = format!(
        "Request Information\n\
         ══════════════════\n\
         Method: {}\n\
         URI: {}\n\
         Headers: {}\n\
         Body Size: {} bytes\n\n\
         Try POST /users with JSON body",
        method, uri, headers_count, body_size
    );

    (StatusCode::OK, info)
}

/// Handler: Create user from JSON body
///
/// This is a real-world pattern showing:
/// 1. Method validation
/// 2. JSON parsing with error handling
/// 3. Data validation
/// 4. Appropriate status codes
/// 5. Response generation
fn create_user_handler(req: Request) -> impl IntoResponse {
    // ✅ STEP 1: Validate HTTP method
    if req.method() != "POST" {
        return (
            StatusCode::METHOD_NOT_ALLOWED,
            serde_json::to_string(&ErrorResponse {
                error: "Method Not Allowed".to_string(),
                details: "Use POST method to create users".to_string(),
            })
            .unwrap(),
        );
    }

    // ✅ STEP 2: Parse JSON body
    let user: User = match serde_json::from_slice(req.body()) {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                serde_json::to_string(&ErrorResponse {
                    error: "Invalid JSON".to_string(),
                    details: format!("Failed to parse request body: {}", e),
                })
                .unwrap(),
            );
        }
    };

    // ✅ STEP 3: Validate data
    if user.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            serde_json::to_string(&ErrorResponse {
                error: "Validation Error".to_string(),
                details: "Name cannot be empty".to_string(),
            })
            .unwrap(),
        );
    }

    if !user.email.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            serde_json::to_string(&ErrorResponse {
                error: "Validation Error".to_string(),
                details: "Email must be valid".to_string(),
            })
            .unwrap(),
        );
    }

    if user.age < 18 {
        return (
            StatusCode::BAD_REQUEST,
            serde_json::to_string(&ErrorResponse {
                error: "Validation Error".to_string(),
                details: "User must be 18 or older".to_string(),
            })
            .unwrap(),
        );
    }

    // ✅ STEP 4: Process (simulate creating user)
    println!("✨ Creating user: {}", user.name);

    let response = UserResponse {
        id: 42, // In real app, this would be from database
        name: user.name,
        email: user.email,
        age: user.age,
        created_at: "2024-01-15T10:30:00Z".to_string(),
        message: "User successfully created!".to_string(),
    };

    // ✅ STEP 5: Return 201 Created with response body
    (
        StatusCode::CREATED,
        serde_json::to_string(&response).unwrap(),
    )
}
