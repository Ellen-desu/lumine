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
/// and executes it.
pub async fn dispatch_request(mut request: Request, app: &Arc<Lumine<Ready>>) -> Response {
    let path = request.uri().path();

    if let Some((route, params)) = app.get_route(path) {
        request.extensions_mut().insert(params);

        let future = if route.middlewares().is_empty() && app.middlewares.is_empty() {
            route.call(request)
        } else {
            let mut chain = Vec::with_capacity(route.middlewares().len() + app.middlewares.len());

            // Choose between route or global middleware which takes precedence
            let iter = if route.run_before_global() {
                route.middlewares().iter().chain(app.middlewares.iter())
            } else {
                app.middlewares.iter().chain(route.middlewares())
            }
            .map(Arc::clone);

            chain.extend(iter);

            let next = Next::new(chain, route.clone());

            Box::pin(next.run(request))
        };

        future.await
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
