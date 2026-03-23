use crate::{
    middleware::Middleware,
    routing::route_service::RouteService,
    types::{request::Request, response::Response, result::Result},
};

/// Represents the remaining execution chain of middlewares.
///
/// `Next` is responsible for advancing the request through the middleware
/// pipeline. It holds a slice of remaining middlewares and the final
/// [`RouteService`] handler.
///
/// This struct is typically passed into [`Middleware::handle`] and is used
/// to delegate execution to the next middleware or the final route handler.
pub struct Next<'a> {
    pub(crate) middlewares: &'a [&'a dyn Middleware],
    pub(crate) route: &'a dyn RouteService,
}

impl<'a> Next<'a> {
    /// Executes the next step in the middleware chain.
    pub fn run(self, request: Request) -> Result<Response> {
        if let Some((first, rest)) = self.middlewares.split_first() {
            let next = Next {
                middlewares: rest,
                route: self.route,
            };

            first.handle(request, next)
        } else {
            self.route.call(request)
        }
    }
}
