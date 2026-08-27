use lumine::prelude::*;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

async fn spawn_test_server(app: Lumine<lumine::application::states::Ready>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        app.serve(listener).await;
    });

    port
}

#[tokio::test]
async fn server_keep_alive() {
    let app = Lumine::builder().route("/echo", async |_| "ok").build();

    let port = spawn_test_server(app).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    // First request
    let req1 = b"GET /echo HTTP/1.1\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n";
    stream.write_all(req1).await.unwrap();

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("ok"));

    // Second request on the same stream
    let req2 = b"GET /echo HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    stream.write_all(req2).await.unwrap();

    let n = stream.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("ok"));

    // Stream should be closed now
    let n = stream.read(&mut buf).await.unwrap();
    assert_eq!(n, 0);
}

#[tokio::test]
async fn server_request_timeout() {
    let timeouts = Timeouts::default().request_read(Duration::from_millis(100));
    let app = Lumine::builder()
        .timeouts(timeouts)
        .route("/", async |_| "ok")
        .build();

    let port = spawn_test_server(app).await;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    // Send partial request and wait for timeout
    let req = b"GET / HTTP/1.1\r\nHost: localhost\r\n";
    stream.write_all(req).await.unwrap();

    // Sleep longer than the timeout
    tokio::time::sleep(Duration::from_millis(150)).await;

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);

    assert!(response.starts_with("HTTP/1.1 408 Request Timeout"));
}

#[tokio::test]
async fn server_max_connections_limit() {
    let limits = Limits::default().max_connections(1);
    let app = Lumine::builder()
        .limits(limits)
        .route("/", async |_| {
            tokio::time::sleep(Duration::from_millis(100)).await;
            "ok"
        })
        .build();

    let port = spawn_test_server(app).await;

    // Open first connection (will block for 100ms processing)
    let mut stream1 = TcpStream::connect(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    let req1 = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    stream1.write_all(req1).await.unwrap();

    // Immediately open second connection
    let mut stream2 = TcpStream::connect(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    // Some OS might need a small delay before the server accepts, but Semaphore try_acquire will fail
    // immediately inside handle_connection.

    let mut buf = [0; 1024];
    let n = stream2.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]);

    assert!(response.starts_with("HTTP/1.1 503 Service Unavailable"));
}
