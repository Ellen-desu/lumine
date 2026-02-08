use pretty_assertions::assert_eq;
use std::net::TcpListener;

use lumine::{Lumine, Params, Query};

// ============================================================================
// 1. PATH PARAMETERS TESTS - SINGLE PARAMETER
// ============================================================================

#[test]
fn test_single_path_parameter() {
    let app = Lumine::builder()
        .route("/users/:id", |req| {
            let params = Params::from_request(&req).unwrap();
            let id = params.get("id").unwrap();
            format!("User ID: {}", id)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10001").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10001/users/123")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "User ID: 123");
}

#[test]
fn test_single_path_parameter_string_value() {
    let app = Lumine::builder()
        .route("/posts/:slug", |req| {
            let params = Params::from_request(&req).unwrap();
            let slug = params.get("slug").unwrap();
            format!("Post slug: {}", slug)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10002").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10002/posts/my-awesome-post")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Post slug: my-awesome-post");
}

#[test]
fn test_single_path_parameter_numeric_string() {
    let app = Lumine::builder()
        .route("/api/v1/items/:item_id", |req| {
            let params = Params::from_request(&req).unwrap();
            let item_id = params.get("item_id").unwrap();

            // Test that we can parse it as a number
            let id: u32 = item_id.parse().unwrap();
            format!("Item ID as number: {}", id)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10003").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10003/api/v1/items/42")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Item ID as number: 42");
}

// ============================================================================
// 2. PATH PARAMETERS TESTS - MULTIPLE PARAMETERS
// ============================================================================

#[test]
fn test_multiple_path_parameters() {
    let app = Lumine::builder()
        .route("/users/:userId/posts/:postId", |req| {
            let params = Params::from_request(&req).unwrap();
            let user_id = params.get("userId").unwrap();
            let post_id = params.get("postId").unwrap();
            format!("User: {}, Post: {}", user_id, post_id)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10004").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10004/users/10/posts/99")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "User: 10, Post: 99");
}

#[test]
fn test_three_path_parameters() {
    let app = Lumine::builder()
        .route("/orgs/:orgId/teams/:teamId/members/:memberId", |req| {
            let params = Params::from_request(&req).unwrap();
            let org_id = params.get("orgId").unwrap();
            let team_id = params.get("teamId").unwrap();
            let member_id = params.get("memberId").unwrap();

            format!("Org: {}, Team: {}, Member: {}", org_id, team_id, member_id)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10005").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10005/orgs/1/teams/5/members/20")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Org: 1, Team: 5, Member: 20");
}

#[test]
fn test_four_path_parameters() {
    let app = Lumine::builder()
        .route("/a/:a/b/:b/c/:c/d/:d", |req| {
            let params = Params::from_request(&req).unwrap();
            let a = params.get("a").unwrap();
            let b = params.get("b").unwrap();
            let c = params.get("c").unwrap();
            let d = params.get("d").unwrap();

            format!("{}-{}-{}-{}", a, b, c, d)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10006").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10006/a/1/b/2/c/3/d/4")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "1-2-3-4");
}

// ============================================================================
// 3. QUERY PARAMETERS TESTS - SINGLE PARAMETER
// ============================================================================

#[test]
fn test_single_query_parameter() {
    let app = Lumine::builder()
        .route("/search", |req| {
            let query = Query::from_request(&req).unwrap();
            let q = query.get("q").unwrap();
            format!("Search for: {}", q[0])
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10007").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10007/search?q=rust")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Search for: rust");
}

#[test]
fn test_multiple_different_query_parameters() {
    let app = Lumine::builder()
        .route("/filter", |req| {
            let query = Query::from_request(&req).unwrap();
            let category = query.get("category").unwrap()[0].clone();
            let sort = query.get("sort").unwrap()[0].clone();

            format!("Category: {}, Sort: {}", category, sort)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10008").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10008/filter?category=books&sort=price")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Category: books, Sort: price");
}

#[test]
fn test_query_parameter_with_special_characters() {
    let app = Lumine::builder()
        .route("/api", |req| {
            let query = Query::from_request(&req).unwrap();
            let name = query.get("name").unwrap()[0].clone();
            format!("Name: {}", name)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10009").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10009/api?name=John%20Doe")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Name: John Doe");
}

// ============================================================================
// 4. QUERY PARAMETERS TESTS - DUPLICATE KEYS
// ============================================================================

#[test]
fn test_duplicate_query_parameters_same_key() {
    let app = Lumine::builder()
        .route("/tags", |req| {
            let query = Query::from_request(&req).unwrap();
            let tags = query.get("tag").unwrap();
            format!("Tags: {}", tags.join(", "))
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10010").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10010/tags?tag=rust&tag=web&tag=api")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Tags: rust, web, api");
}

#[test]
fn test_duplicate_query_parameters_count() {
    let app = Lumine::builder()
        .route("/list", |req| {
            let query = Query::from_request(&req).unwrap();
            let items = query.get("item").unwrap();
            format!("Item count: {}", items.len())
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10011").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10011/list?item=a&item=b&item=c&item=d")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Item count: 4");
}

#[test]
fn test_mixed_duplicate_and_unique_query_parameters() {
    let app = Lumine::builder()
        .route("/advanced", |req| {
            let query = Query::from_request(&req).unwrap();
            let page = query.get("page").unwrap()[0].clone();
            let filters = query.get("filter").unwrap();

            format!("Page: {}, Filters: {}", page, filters.join("|"))
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10012").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response =
        ureq::get("http://127.0.0.1:10012/advanced?page=2&filter=active&filter=verified")
            .call()
            .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Page: 2, Filters: active|verified");
}

// ============================================================================
// 5. PATH + QUERY PARAMETERS TESTS
// ============================================================================

#[test]
fn test_path_and_query_parameters_combined() {
    let app = Lumine::builder()
        .route("/users/:userId/posts", |req| {
            let path_params = Params::from_request(&req).unwrap();
            let query_params = Query::from_request(&req).unwrap();

            let user_id = path_params.get("userId").unwrap();
            let sort = query_params.get("sort").unwrap()[0].clone();

            format!("User: {}, Sort: {}", user_id, sort)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10013").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10013/users/5/posts?sort=date")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "User: 5, Sort: date");
}

#[test]
fn test_complex_path_and_query_combination() {
    let app = Lumine::builder()
        .route("/api/v1/orgs/:orgId/users/:userId", |req| {
            let path_params = Params::from_request(&req).unwrap();
            let query_params = Query::from_request(&req).unwrap();

            let org_id = path_params.get("orgId").unwrap();
            let user_id = path_params.get("userId").unwrap();
            let fields = query_params.get("fields").unwrap();
            let include = query_params.get("include").unwrap()[0].clone();

            format!(
                "Org: {}, User: {}, Fields: [{}], Include: {}",
                org_id,
                user_id,
                fields.join(","),
                include
            )
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10014").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get(
        "http://127.0.0.1:10014/api/v1/orgs/123/users/456?fields=name&fields=email&include=profile",
    )
    .call()
    .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(
        body,
        "Org: 123, User: 456, Fields: [name,email], Include: profile"
    );
}

// ============================================================================
// 6. EMPTY/MISSING PARAMETERS TESTS
// ============================================================================

#[test]
fn test_route_without_parameters() {
    let app = Lumine::builder()
        .route("/static", |req| {
            let params = Params::from_request(&req);
            if params.is_none() {
                "No params"
            } else {
                "Has params"
            }
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10015").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10015/static").call().unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Has params");
}

#[test]
fn test_query_parameters_none_when_no_query_string() {
    let app = Lumine::builder()
        .route("/list", |req| {
            let query = Query::from_request(&req);
            if query.is_none() {
                "No query"
            } else {
                "Has query"
            }
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10016").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10016/list").call().unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Has query");
}

#[test]
fn test_accessing_nonexistent_parameter() {
    let app = Lumine::builder()
        .route("/items/:id", |req| {
            let params = Params::from_request(&req).unwrap();
            let name = params.get("name"); // This doesn't exist

            if name.is_none() {
                "Parameter not found"
            } else {
                "Parameter found"
            }
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10017").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10017/items/123")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Parameter not found");
}

// ============================================================================
// 7. PARAMETER VALUE PARSING TESTS
// ============================================================================

#[test]
fn test_parse_parameter_as_u32() {
    let app = Lumine::builder()
        .route("/numbers/:value", |req| {
            let params = Params::from_request(&req).unwrap();
            let value_str = params.get("value").unwrap();
            let value: u32 = value_str.parse().unwrap();

            format!("Double: {}", value * 2)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10018").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10018/numbers/21")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Double: 42");
}

#[test]
fn test_parse_parameter_as_i32() {
    let app = Lumine::builder()
        .route("/temperature/:value", |req| {
            let params = Params::from_request(&req).unwrap();
            let value_str = params.get("value").unwrap();
            let value: i32 = value_str.parse().unwrap();

            format!("Celsius: {}", value)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10019").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10019/temperature/-15")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Celsius: -15");
}

#[test]
fn test_parse_parameter_as_f64() {
    let app = Lumine::builder()
        .route("/price/:amount", |req| {
            let params = Params::from_request(&req).unwrap();
            let amount_str = params.get("amount").unwrap();
            let amount: f64 = amount_str.parse().unwrap();

            format!("Total: {:.2}", amount * 1.1) // Add 10% tax
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10020").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10020/price/100.50")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert!(body.starts_with("Total: 110"));
}

// ============================================================================
// 8. URL ENCODED PARAMETERS TESTS
// ============================================================================

#[test]
fn test_query_parameter_with_equals_sign() {
    let app = Lumine::builder()
        .route("/config", |req| {
            let query = Query::from_request(&req).unwrap();
            let setting = query.get("setting").unwrap()[0].clone();
            format!("Setting: {}", setting)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10021").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Note: URL encoding converts = to %3D
    let response = ureq::get("http://127.0.0.1:10021/config?setting=key%3Dvalue")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Setting: key=value");
}

#[test]
fn test_query_parameter_with_ampersand() {
    let app = Lumine::builder()
        .route("/expr", |req| {
            let query = Query::from_request(&req).unwrap();
            let expr = query.get("expr").unwrap()[0].clone();
            format!("Expression: {}", expr)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10022").unwrap();
    let _rx = app.serve(listener).unwrap();

    // Ampersand encoded as %26
    let response = ureq::get("http://127.0.0.1:10022/expr?expr=a%26b%26c")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Expression: a&b&c");
}

// ============================================================================
// 9. EDGE CASES
// ============================================================================

#[test]
fn test_parameter_with_hyphens() {
    let app = Lumine::builder()
        .route("/posts/:post-slug", |req| {
            let params = Params::from_request(&req).unwrap();
            let slug = params.get("post-slug").unwrap();
            format!("Slug: {}", slug)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10023").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10023/posts/my-awesome-blog-post")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Slug: my-awesome-blog-post");
}

#[test]
fn test_parameter_with_underscores() {
    let app = Lumine::builder()
        .route("/user/:user_name", |req| {
            let params = Params::from_request(&req).unwrap();
            let name = params.get("user_name").unwrap();
            format!("Username: {}", name)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10024").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10024/user/john_doe_123")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Username: john_doe_123");
}

#[test]
fn test_parameter_with_numbers_and_letters() {
    let app = Lumine::builder()
        .route("/v/:version", |req| {
            let params = Params::from_request(&req).unwrap();
            let version = params.get("version").unwrap();
            format!("Version: {}", version)
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10025").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10025/v/v2beta3")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    assert_eq!(body, "Version: v2beta3");
}

#[test]
fn test_empty_query_parameter_value() {
    let app = Lumine::builder()
        .route("/search", |req| match Query::from_request(&req) {
            Some(query) => match query.get("q") {
                Some(q) => format!("Query found, values: {}", q.len()),
                _ => "Query param not found".to_string(),
            },
            _ => "No query".to_string(),
        })
        .build();

    let listener = TcpListener::bind("127.0.0.1:10026").unwrap();
    let _rx = app.serve(listener).unwrap();

    let response = ureq::get("http://127.0.0.1:10026/search?q=")
        .call()
        .unwrap();
    let body = response.into_body().read_to_string().unwrap();

    // Empty values will still be added to the query
    assert_eq!(body, "Query found, values: 1");
}
