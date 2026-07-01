use async_trait::async_trait;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::{TlsAcceptor, rustls::ServerConfig};

use crate::{
    application::{lumine::Lumine, states::Ready},
    internal::connection,
};

#[async_trait]
pub trait TlsExt {
    async fn serve_tls(self, listener: TcpListener, config: ServerConfig);
}

#[async_trait]
impl TlsExt for Lumine<Ready> {
    async fn serve_tls(self, listener: TcpListener, config: ServerConfig) {
        let app = Arc::new(self);
        let acceptor = TlsAcceptor::from(Arc::new(config));

        loop {
            if let Ok((stream, _)) = listener.accept().await
                && let Ok(tls_stream) = acceptor.accept(stream).await
            {
                let app = Arc::clone(&app);
                tokio::spawn(async move { connection::handle_connection(app, tls_stream).await });
            }
        }
    }
}
