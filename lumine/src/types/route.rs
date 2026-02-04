use crate::traits::route_service::RouteService;

pub(crate) type RouteType = Box<dyn RouteService + Send + Sync>;
