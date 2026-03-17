use crate::{
    routing::{params::Params, path::Path},
    traits::{into_response::IntoResponse, route_service::RouteService},
    types::{request::Request, response::Response, result::Result},
};

pub(crate) struct Route<'a, F> {
    pub(crate) path: Path<'a>,
    pub(crate) handler: F,
}

impl<'a, F, R> RouteService for Route<'a, F>
where
    F: Fn(Request) -> R + Send + Sync + 'static,
    R: IntoResponse,
{
    fn matches(&self, path: &Path) -> Option<Params> {
        if path.len() != self.path.len() {
            None
        } else {
            let mut params = Params::default();

            for (route_part, path_parts) in self.path.iter().zip(path.as_ref()) {
                if route_part.starts_with(":") {
                    // If the route path is starts with ":", then take it as parameter
                    params.insert(
                        route_part.strip_prefix(":").unwrap().to_owned(),
                        (*path_parts).into(),
                    );
                } else if route_part != path_parts {
                    return None;
                }
            }

            Some(params)
        }
    }
    fn is_duplicated(&self, path: &Path) -> bool {
        *self.path == **path
    }
    fn call(&self, request: Request) -> Result<Response> {
        Ok((self.handler)(request).into_response()?)
    }
}
