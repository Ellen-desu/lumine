//! # Route Parameters Example
//!
//! Demonstrates extracting path parameters and query parameters from requests.
//!
//! ## What You'll Learn
//! - Path parameters (e.g., /users/:id)
//! - Query parameters (e.g., ?search=term&limit=10)
//! - Multiple path parameters
//! - Parameter extraction and usage
//! - Building real-world APIs with dynamic routes
//!
//! ## Try It
//! ```bash
//! cargo run --example parameters
//!
//! # Path parameters
//! curl http://127.0.0.1:8080/users/123
//! curl http://127.0.0.1:8080/users/456
//!
//! # Query parameters
//! curl "http://127.0.0.1:8080/search?q=rust&limit=5"
//!
//! # Multiple path parameters
//! curl http://127.0.0.1:8080/users/123/posts/456
//!
//! # Combination of path and query
//! curl "http://127.0.0.1:8080/users/123/posts?sort=date&limit=10"
//! ```

use lumine::prelude::*;
use serde::Serialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Lumine::builder()
        // Single path parameter
        .route("/users/:id", get_user_handler)
        // Query parameters
        .route("/search", search_handler)
        // Multiple path parameters
        .route("/users/:user_id/posts/:postId", get_post_handler)
        // Multiple path parameters with query
        .route("/users/:user_id/posts", list_posts_handler)
        // Root endpoint
        .route("/", root_handler)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("✅ Server running at http://127.0.0.1:8080");
    println!("\n📍 Parameter Examples:");
    println!("   GET /users/:id                    → Get user by ID");
    println!("   GET /search?q=term&limit=N        → Search with query params");
    println!("   GET /users/:id/posts/:id          → Get specific post");
    println!("   GET /users/:id/posts?sort=date    → List posts with query");
    println!("\n💡 Try:");
    println!("   curl http://127.0.0.1:8080/users/123");
    println!("   curl \"http://127.0.0.1:8080/search?q=rust&limit=5\"");
    println!("   curl http://127.0.0.1:8080/users/10/posts/20");
    println!("   curl \"http://127.0.0.1:8080/users/10/posts?sort=date\"");
    println!("\n⏹️  Press Ctrl+C to stop\n");

    app.serve(listener).await;

    Ok(())
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Serialize)]
struct UserData {
    id: u32,
    name: String,
    email: String,
    followers: u32,
}

#[derive(Debug, Serialize)]
struct PostData {
    id: u32,
    user_id: u32,
    title: String,
    content: String,
    likes: u32,
}

#[derive(Debug, Serialize)]
struct SearchResult {
    query: String,
    limit: usize,
    results: Vec<String>,
}

// ============================================================================
// Handlers
// ============================================================================

/// Handler: Root endpoint
async fn root_handler(_: Request) -> impl IntoResponse {
    "Welcome to Parameters Example!\n\n\
     Try:\n\
     • /users/123\n\
     • /search?q=rust&limit=5\n\
     • /users/10/posts/20"
}

/// Handler: Get user by ID from path parameter
///
/// URL: /users/:id
/// Example: /users/123
async fn get_user_handler(req: Request) -> impl IntoResponse {
    // Extract path parameters
    let params = Params::from_request(&req);

    // Get the 'id' parameter
    let user_id = match params.get("id") {
        Some(id) => id,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "Missing 'id' parameter".to_string(),
            );
        }
    };

    // Parse as number (optional, depends on your use case)
    let id_num = match user_id.parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Invalid user ID: '{}' is not a number", user_id),
            );
        }
    };

    // Simulate fetching user data
    let user = UserData {
        id: id_num,
        name: format!("User #{}", id_num),
        email: format!("user{}@example.com", id_num),
        followers: id_num * 100,
    };

    // Return as JSON
    (StatusCode::OK, serde_json::to_string(&user).unwrap())
}

/// Handler: Search with query parameters
///
/// URL: /search?q=term&limit=N
/// Example: /search?q=rust&limit=5
async fn search_handler(req: Request) -> impl IntoResponse {
    // Extract query parameters
    let query_params = Query::from_request(&req);

    // Get 'q' parameter (search query)
    let search_term = match query_params.get("q") {
        Some(terms) if !terms.is_empty() => terms[0].clone(),
        _ => {
            return (StatusCode::BAD_REQUEST, "Missing 'q' parameter".to_string());
        }
    };

    // Get 'limit' parameter with default
    let limit = match query_params.get("limit") {
        Some(limits) if !limits.is_empty() => limits[0].parse::<usize>().unwrap_or(10),
        _ => 10, // Default limit
    };

    // Simulate search results
    let results = SearchResult {
        query: search_term.clone(),
        limit,
        results: vec![
            format!("Result 1 for '{}'", search_term),
            format!("Result 2 for '{}'", search_term),
            format!("Result 3 for '{}'", search_term),
        ],
    };

    (StatusCode::OK, serde_json::to_string(&results).unwrap())
}

/// Handler: Get specific post by user and post IDs
///
/// URL: /users/:user_id/posts/:postId
/// Example: /users/10/posts/20
async fn get_post_handler(req: Request) -> impl IntoResponse {
    let params = Params::from_request(&req);

    // Get user ID
    let user_id = match params.get("user_id") {
        Some(uid) => match uid.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid user_id".to_string()),
        },
        None => return (StatusCode::BAD_REQUEST, "Missing user_id".to_string()),
    };

    // Get post ID
    let post_id = match params.get("postId") {
        Some(pid) => match pid.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid postId".to_string()),
        },
        None => return (StatusCode::BAD_REQUEST, "Missing postId".to_string()),
    };

    // Simulate fetching post
    let post = PostData {
        id: post_id,
        user_id,
        title: format!("Post #{} by User #{}", post_id, user_id),
        content: "This is post content...".to_string(),
        likes: (user_id + post_id) * 50,
    };

    (StatusCode::OK, serde_json::to_string(&post).unwrap())
}

/// Handler: List posts by user with optional query parameters
///
/// URL: /users/:user_id/posts?sort=date&limit=N
/// Example: /users/10/posts?sort=date&limit=5
async fn list_posts_handler(req: Request) -> impl IntoResponse {
    // Get path parameter
    let params = Params::from_request(&req);

    let user_id = match params.get("user_id") {
        Some(uid) => match uid.parse::<u32>() {
            Ok(n) => n,
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid user_id".to_string()),
        },
        None => return (StatusCode::BAD_REQUEST, "Missing user_id".to_string()),
    };

    // Get query parameters
    let query_params = Query::from_request(&req);

    // Extract sort parameter (default: "recent")
    let sort = query_params
        .get("sort")
        .and_then(|v| v.first().cloned())
        .unwrap_or_else(|| "recent".to_string());

    // Extract limit parameter (default: 10)
    let limit = query_params
        .get("limit")
        .and_then(|v| v.first().cloned())
        .and_then(|l| l.parse::<u32>().ok())
        .unwrap_or(10);

    // Build response showing extracted parameters
    let response = format!(
        "Posts for User #{}\n\
         ═══════════════════\n\
         Sort: {}\n\
         Limit: {}\n\n\
         📝 Posts:\n\
         • Post #1: 'First Post' (150 likes)\n\
         • Post #2: 'Second Post' (200 likes)\n\
         • Post #3: 'Third Post' (180 likes)",
        user_id, sort, limit
    );

    (StatusCode::OK, response)
}
