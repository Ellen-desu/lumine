//! Request dispatching module.
//!
//! This module provides functionality for finding the appropriate route for a request,
//! executing the middleware chain, and generating the final response.

use crate::{
    application::{lumine::Lumine, states::Ready},
    middleware::next::Next,
    request::Request,
    response::{Response, into_response::IntoResponse},
};
use http::StatusCode;
use std::sync::Arc;

/// Dispatches a request to the appropriate route.
///
/// This function finds the matching route, prepares the middleware chain,
/// and executes it. It also includes panic handling to ensure the application
/// doesn't crash on handler errors.
pub async fn dispatch_request(mut request: Request, app: &Arc<Lumine<Ready>>) -> Response {
    let status = match app.get_route(request.uri().path()) {
        Some((route, params)) => {
            request.extensions_mut().insert(params);

            let mut chain = Vec::with_capacity(route.middlewares().len() + app.middlewares.len());

            // Choose between route or global middleware which takes precedence
            let iter = if route.run_before_global() {
                route.middlewares().iter().chain(app.middlewares.iter())
            } else {
                app.middlewares.iter().chain(route.middlewares())
            }
            .map(Arc::clone);

            chain.extend(iter);

            let next = Next {
                middlewares: chain,
                route: Arc::clone(route),
                index: 0,
            };

            match tokio::spawn(async { next.run(request).await }).await {
                Ok(response) => return response,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
        _ => StatusCode::NOT_FOUND,
    };

    status.into_response()
}
