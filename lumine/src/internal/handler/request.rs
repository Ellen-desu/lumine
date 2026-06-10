use crate::{
    application::{lumine::Lumine, states::Ready},
    body::Body,
    middleware::next::Next,
    types::{request::Request, response::Response, result::Result},
};
use http::StatusCode;
use std::{
    panic::{self, AssertUnwindSafe},
    sync::Arc,
};

pub fn dispatch_request(mut request: Request, app: &Arc<Lumine<Ready>>) -> Result<Response> {
    let response = match app.get_route(request.uri()) {
        Some((route, params)) => {
            request.extensions_mut().insert(params);

            let mut chain = Vec::new();

            // Choose between route or global middleware which takes precedence
            let iter = if route.route_middleware_first() {
                route.middlewares().iter().chain(app.middlewares.iter())
            } else {
                app.middlewares.iter().chain(route.middlewares())
            }
            .map(|b| b.as_ref());

            chain.extend(iter);

            let next = Next {
                middlewares: &chain,
                route,
            };

            // Start the middleware chain and catch the panic to prevent app from crash
            match panic::catch_unwind(AssertUnwindSafe(|| next.run(request).unwrap())) {
                Ok(response) => response,
                _ => http::Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::Empty)?,
            }
        }
        _ => http::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::Empty)?,
    };

    Ok(response)
}
