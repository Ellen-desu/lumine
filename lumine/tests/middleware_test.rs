#[cfg(test)]
mod global_middleware {
    use lumine::{
        Lumine, Middleware, Request,
        http::{HeaderValue, StatusCode},
    };
    use std::net::TcpListener;

    struct SecretTokenChecker {
        secret: &'static str,
    }

    impl Middleware for SecretTokenChecker {
        fn handle(&self, request: Request, next: lumine::Next) -> lumine::Result<lumine::Response> {
            if let Some(secret) = request.headers().get("SECRET")
                && secret == self.secret
            {
                next.run(request)
            } else {
                let mut response = next.run(request)?;
                *response.status_mut() = StatusCode::UNAUTHORIZED;

                Ok(response)
            }
        }
    }

    #[test]
    fn test_single_middleware() {
        let app = Lumine::builder()
            .route("/", |_| "Hello, World!")
            .middleware(SecretTokenChecker { secret: "LUMINE" })
            .build();
        let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

        app.serve(listener);

        assert!(
            ureq::get("http://127.0.0.1:8000")
                .header("SECRET", "LUMINE")
                .call()
                .is_ok(),
        );

        // Should be error because doesn't have secret key
        assert!(ureq::get("http://127.0.0.1:8000").call().is_err());
    }

    struct SecretTokenApplier;

    impl Middleware for SecretTokenApplier {
        fn handle(
            &self,
            mut request: Request,
            next: lumine::Next,
        ) -> lumine::Result<lumine::Response> {
            request
                .headers_mut()
                .append("SECRET", HeaderValue::from_static("LUMINE"));

            next.run(request)
        }
    }

    #[test]
    fn test_double_middleware() {
        let app = Lumine::builder()
            .route("/", |_| "Hello, World!")
            .middleware(SecretTokenApplier)
            .middleware(SecretTokenChecker { secret: "LUMINE" })
            .build();
        let listener = TcpListener::bind("127.0.0.1:8001").unwrap();

        app.serve(listener);

        assert!(ureq::get("http://127.0.0.1:8001").call().is_ok());
    }

    #[test]
    fn test_reorder_middleware() {
        let app = Lumine::builder()
            .route("/", |_| "Hello, World!")
            .middleware(SecretTokenChecker { secret: "LUMINE" }) // Reordered
            .middleware(SecretTokenApplier)
            .build();
        let listener = TcpListener::bind("127.0.0.1:8002").unwrap();

        app.serve(listener);

        assert!(ureq::get("http://127.0.0.1:8002").call().is_err());
    }
}

#[cfg(test)]
mod route_middleware {
    use std::net::TcpListener;

    use lumine::{
        Lumine, Middleware,
        http::{StatusCode, header::AUTHORIZATION},
    };

    struct AdminChecker;

    impl Middleware for AdminChecker {
        fn handle(
            &self,
            request: lumine::Request,
            next: lumine::Next,
        ) -> lumine::Result<lumine::Response> {
            if let Some(auth) = request.headers().get(AUTHORIZATION)
                && auth == "ADMIN"
            {
                next.run(request)
            } else {
                let mut response = next.run(request)?;
                *response.status_mut() = StatusCode::UNAUTHORIZED;

                Ok(response)
            }
        }
    }

    #[test]
    fn test_single_middleware() {
        let app = Lumine::builder()
            .route_with(
                "/admin",
                |_| "Hello, Admin!",
                |r| r.middleware(AdminChecker),
            )
            .build();
        let listener = TcpListener::bind("127.0.0.1:8003").unwrap();

        app.serve(listener);

        assert!(
            ureq::get("http://127.0.0.1:8003/admin")
                .header(AUTHORIZATION, "ADMIN")
                .call()
                .is_ok()
        );
    }
}
