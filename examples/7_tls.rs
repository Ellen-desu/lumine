//! # TLS Example
//!
//! How to run a secure Lumine server with TLS/HTTPS.
//!
//! ## What You'll Learn
//! - Activating the `tls` feature in Lumine.
//! - Loading TLS certificates and keys using `rustls-pki-types`.
//! - Starting the server with `serve_tls`.
//!
//! ## Prerequisites
//! 1. Ensure the `tls` feature is enabled for `lumine`.
//! 2. Ensure `tokio-rustls` and `rustls-pki-types` are added to your `Cargo.toml`.
//! 3. Generate certificates with `mkcert`:
//!    ```bash
//!    mkcert -install
//!    mkcert 127.0.0.1
//!    # This creates 127.0.0.1.pem and 127.0.0.1-key.pem in your project root
//!    ```
//!
//! ## Try It
//! ```bash
//! cargo run --example tls
//! curl https://127.0.0.1:8080
//! ```

use lumine::{Lumine, tls::TlsExt};
use rustls_pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject};
use tokio::net::TcpListener;
use tokio_rustls::rustls::ServerConfig;

#[tokio::main]
async fn main() {
    let app = Lumine::builder()
        .route("/", async |_| "Hello, World!")
        .build();
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let certs = vec![CertificateDer::from_pem_file("127.0.0.1.pem").unwrap()];
    let key = PrivateKeyDer::from_pem_file("127.0.0.1-key.pem").unwrap();

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .unwrap();

    println!("✅ Server running at https://127.0.0.1:8080");
    println!("💡 Try: curl https://127.0.0.1:8080");
    println!("⏹️  Press Ctrl+C to stop\n");

    app.serve_tls(listener, config).await.unwrap();
}
