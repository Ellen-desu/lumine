use lumine::Middleware;

pub struct HeaderModifier;

impl Middleware for HeaderModifier {
    fn handle(
        &self,
        mut request: lumine::Request,
        next: lumine::Next,
    ) -> lumine::Result<lumine::Response> {
        request.headers_mut().append("x-test", 123.into());

        let mut response = next.run(request).unwrap();

        response.headers_mut().append("x-test", 123.into());

        Ok(response)
    }
}
