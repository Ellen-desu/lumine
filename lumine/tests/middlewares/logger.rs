use lumine::Middleware;
use std::sync::{Arc, Mutex};

pub struct LoggerA {
    pub log: Arc<Mutex<Vec<&'static str>>>,
}

pub struct LoggerB {
    pub log: Arc<Mutex<Vec<&'static str>>>,
}

pub struct LoggerC {
    pub log: Arc<Mutex<Vec<&'static str>>>,
}

pub struct LoggerD {
    pub log: Arc<Mutex<Vec<&'static str>>>,
}

impl Middleware for LoggerA {
    fn handle(
        &self,
        request: lumine::Request,
        next: lumine::Next,
    ) -> lumine::Result<lumine::Response> {
        self.log.lock().unwrap().push("A before");

        let response = next.run(request);

        self.log.lock().unwrap().push("A after");

        response
    }
}

impl Middleware for LoggerB {
    fn handle(
        &self,
        request: lumine::Request,
        next: lumine::Next,
    ) -> lumine::Result<lumine::Response> {
        self.log.lock().unwrap().push("B before");

        let response = next.run(request);

        self.log.lock().unwrap().push("B after");

        response
    }
}

impl Middleware for LoggerC {
    fn handle(
        &self,
        request: lumine::Request,
        next: lumine::Next,
    ) -> lumine::Result<lumine::Response> {
        self.log.lock().unwrap().push("C before");

        let response = next.run(request);

        self.log.lock().unwrap().push("C after");

        response
    }
}
impl Middleware for LoggerD {
    fn handle(
        &self,
        request: lumine::Request,
        next: lumine::Next,
    ) -> lumine::Result<lumine::Response> {
        self.log.lock().unwrap().push("D before");

        let response = next.run(request);

        self.log.lock().unwrap().push("D after");

        response
    }
}
