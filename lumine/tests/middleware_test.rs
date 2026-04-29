mod middlewares;

use lumine::{Lumine, http::HeaderValue};
use middlewares::*;
use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};

use pretty_assertions::assert_eq;

#[test]
fn middleware_order_global() {
    let log = Arc::new(Mutex::new(Vec::new()));

    let app = Lumine::builder()
        .route("/", |_| ())
        .middleware(LoggerA { log: log.clone() })
        .middleware(LoggerB { log: log.clone() })
        .build();
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    app.serve(listener);
    ureq::get("http://127.0.0.1:8000").call().unwrap();

    assert_eq!(
        *log.lock().unwrap(),
        ["A before", "B before", "B after", "A after"]
    );
}

#[test]
fn middleware_order_specific() {
    let log = Arc::new(Mutex::new(Vec::new()));

    let cloned = log.clone();

    let app = Lumine::builder()
        .route_with(
            "/",
            |_| (),
            move |r| {
                r.middleware(LoggerA {
                    log: cloned.clone(),
                })
                .middleware(LoggerB {
                    log: cloned.clone(),
                })
            },
        )
        .build();
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();

    app.serve(listener);
    ureq::get("http://127.0.0.1:8001").call().unwrap();

    assert_eq!(
        *log.lock().unwrap(),
        ["A before", "B before", "B after", "A after"]
    );
}

#[test]
fn middleware_order_specific_and_global() {
    let log = Arc::new(Mutex::new(Vec::new()));

    let cloned = log.clone();

    let app = Lumine::builder()
        .route_with(
            "/",
            |_| (),
            move |r| {
                r.middleware(LoggerC {
                    log: cloned.clone(),
                })
                .middleware(LoggerD {
                    log: cloned.clone(),
                })
            },
        )
        .middleware(LoggerA { log: log.clone() })
        .middleware(LoggerB { log: log.clone() })
        .build();
    let listener = TcpListener::bind("127.0.0.1:8002").unwrap();

    app.serve(listener);
    ureq::get("http://127.0.0.1:8002").call().unwrap();

    assert_eq!(
        *log.lock().unwrap(),
        [
            "A before", "B before", "C before", "D before", "D after", "C after", "B after",
            "A after"
        ]
    );
}

#[test]
fn middleware_order_specific_and_global_reversed() {
    let log = Arc::new(Mutex::new(Vec::new()));

    let cloned = log.clone();

    let app = Lumine::builder()
        .route_with(
            "/",
            |_| (),
            move |r| {
                r.middleware(LoggerC {
                    log: cloned.clone(),
                })
                .middleware(LoggerD {
                    log: cloned.clone(),
                })
                .route_middleware_first()
            },
        )
        .middleware(LoggerA { log: log.clone() })
        .middleware(LoggerB { log: log.clone() })
        .build();
    let listener = TcpListener::bind("127.0.0.1:8003").unwrap();

    app.serve(listener);
    ureq::get("http://127.0.0.1:8003").call().unwrap();

    assert_eq!(
        *log.lock().unwrap(),
        [
            "C before", "D before", "A before", "B before", "B after", "A after", "D after",
            "C after"
        ]
    );
}

#[test]
fn header_modification() {
    let app = Lumine::builder()
        .route("/", |r| {
            assert_eq!(r.headers().get("x-test").unwrap().to_str().unwrap(), "123")
        })
        .middleware(HeaderModifier)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8004").unwrap();

    app.serve(listener);

    // Check whether the response has 'x-test' header or not.
    assert_eq!(
        ureq::get("http://127.0.0.1:8004")
            .call()
            .unwrap()
            .headers()
            .get("x-test")
            .unwrap(),
        HeaderValue::from(123)
    );
}

#[test]
fn middleware_error() {
    let app = Lumine::builder()
        .route("/", |_| ())
        .middleware(MiddlewareError)
        .build();

    let listener = TcpListener::bind("127.0.0.1:8005").unwrap();

    app.serve(listener);

    assert!(matches!(
        ureq::get("http://127.0.0.1:8005").call(),
        Err(ureq::Error::StatusCode(500))
    ));
}
