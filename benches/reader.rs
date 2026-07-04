use criterion::{Criterion, criterion_group, criterion_main};
use lumine::{internal::reader::read_request, prelude::*};
use std::{hint::black_box, io::Cursor};
use tokio::io::BufReader;

fn make_request(body_size: usize) -> Vec<u8> {
    let body = vec![b'a'; body_size];

    format!(
        "POST /upload HTTP/1.1\r\n\
         Accept: */*\r\n\
         Host: localhost\r\n\
         Accept-Encoding: gzip, deflate, br, zstd\r\n\
         Content-Length: {}\r\n\
         Connection: keep-alive\r\n\
         Accept-Language: en-US,en;q=0.9\r\n\
         Content-Type: text/plain\r\n\
         User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:152.0) Gecko/20100101 Firefox/152.0\r\n\
         \r\n",
        body_size
    )
    .into_bytes()
    .into_iter()
    .chain(body)
    .collect()
}

fn make_get_request(headers: usize) -> Vec<u8> {
    let mut req = String::new();

    req.push_str("GET /search?q=rust HTTP/1.1\r\n");
    req.push_str("Host: localhost\r\n");

    for i in 0..(headers - 1) {
        req.push_str(&format!(
            "X-Custom-Header-{}: some-long-header-value-{}\r\n",
            i, i
        ));
    }

    req.push_str("\r\n");

    req.into_bytes()
}

fn benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    for count in [4, 16, 32, 64, 100] {
        let req = make_get_request(count);

        c.bench_function(&format!("headers_{}", count), |b| {
            b.to_async(&rt).iter(|| async {
                let cursor = Cursor::new(&req);
                let mut reader = BufReader::new(cursor);

                let req = read_request(&mut reader, &Limits::default()).await.unwrap();

                black_box(req);
            });
        });
    }

    for size in [128, 1024, 16 * 1024, 1024 * 1024] {
        let req = make_request(size);

        c.bench_function(&format!("body_{}b", size), |b| {
            b.to_async(&rt).iter(|| async {
                let cursor = Cursor::new(&req);
                let mut reader = BufReader::new(cursor);

                let parsed = read_request(&mut reader, &Limits::default()).await.unwrap();

                black_box(parsed);
            });
        });
    }
}

criterion_group!(bench, benchmark);
criterion_main!(bench);
