use lumine::Middleware;

pub struct MiddlewareError;

impl Middleware for MiddlewareError {
    fn handle(&self, _: lumine::Request, _: lumine::Next) -> lumine::Result<lumine::Response> {
        Err(lumine::Error::Parser)
    }
}
