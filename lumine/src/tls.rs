use async_trait::async_trait;
use std::{io::Result, sync::Arc};
use tokio::net::TcpListener;
use tokio_rustls::{TlsAcceptor, rustls::ServerConfig};

use crate::{
    application::{lumine::Lumine, states::Ready},
    internal::connection,
};

#[async_trait]
pub trait TlsExt {
    async fn serve_tls(self, listener: TcpListener, config: ServerConfig) -> Result<()>;
}

#[async_trait]
impl TlsExt for Lumine<Ready> {
    async fn serve_tls(self, listener: TcpListener, config: ServerConfig) -> Result<()> {
        let app = Arc::new(self);
        let acceptor = TlsAcceptor::from(Arc::new(config));

        loop {
            let (stream, _) = listener.accept().await?;

            let app = Arc::clone(&app);
            let acceptor = acceptor.clone();

            tokio::spawn(async move {
                if let Ok(tls_stream) = acceptor.accept(stream).await {
                    connection::handle_connection(app, tls_stream).await
                }
            });
        }
    }
}
