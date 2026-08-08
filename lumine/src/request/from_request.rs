use crate::request::{Request, params::Params, query::Query};

pub trait FromRequest {
    /// Retrieves a value from the request.
    fn from_request(req: &Request) -> &Self;
}

impl FromRequest for Params {
    /// Retrieves path parameters from the request extensions.
    fn from_request(req: &Request) -> &Self {
        req.extensions()
            .get::<Self>()
            .expect("path parameters are always attached")
    }
}

impl FromRequest for Query {
    /// Retrieves query parameters from the request extensions.
    fn from_request(req: &Request) -> &Self {
        req.extensions()
            .get::<Self>()
            .expect("query parameters are always attached")
    }
}
