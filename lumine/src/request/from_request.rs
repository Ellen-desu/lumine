use crate::request::{
    Request,
    extensions::{addr::Addr, params::Params, query::Query, remainder::Remainder},
};

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

impl FromRequest for Addr {
    /// Retrieves query parameters from the request extensions.
    fn from_request(req: &Request) -> &Self {
        req.extensions()
            .get::<Self>()
            .expect("address is always attached")
    }
}

impl FromRequest for Remainder {
    /// Retrieves the remainder of the request path from the request extensions.
    fn from_request(req: &Request) -> &Self {
        req.extensions()
            .get::<Self>()
            .expect("remainder is always attached")
    }
}
