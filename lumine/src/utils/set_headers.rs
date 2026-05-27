use http::{HeaderMap, request, response};

pub trait SetHeaders {
    fn headers(self, headers: &HeaderMap) -> Self;
}

impl SetHeaders for response::Builder {
    fn headers(mut self, headers: &HeaderMap) -> Self {
        for (key, value) in headers.iter() {
            self = self.header(key, value);
        }

        self
    }
}

impl SetHeaders for request::Builder {
    fn headers(mut self, headers: &HeaderMap) -> Self {
        for (key, value) in headers.iter() {
            self = self.header(key, value);
        }

        self
    }
}
