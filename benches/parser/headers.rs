use criterion::{Criterion, criterion_group, criterion_main};
use lumine::{internal::parser, prelude::*};
use rand::seq::SliceRandom;
use std::hint::black_box;

fn generate_headers(n: usize) -> Vec<String> {
    const HEADER_NAMES: &[&str] = &[
        "Host",
        "User-Agent",
        "Accept",
        "Accept-Encoding",
        "Accept-Language",
        "Connection",
        "Cache-Control",
        "Cookie",
        "Referer",
        "Authorization",
        "Content-Type",
        "Content-Length",
        "Origin",
        "Upgrade-Insecure-Requests",
        "X-Forwarded-For",
        "X-Request-Id",
        "If-None-Match",
        "ETag",
        "Pragma",
        "DNT",
    ];

    const HEADER_VALUES: &[&str] = &[
        "localhost",
        "keep-alive",
        "gzip, deflate, br",
        "en-US,en;q=0.9",
        "application/json",
        "text/html",
        "Mozilla/5.0",
        "max-age=0",
        "no-cache",
        "Bearer abc123xyz",
        "sessionid=deadbeef",
        "chunked",
        "rust-client/0.1",
        "127.0.0.1",
        "*/*",
    ];

    let mut headers = Vec::with_capacity(n);

    for i in 0..n {
        let name = HEADER_NAMES[i % HEADER_NAMES.len()];
        let value = HEADER_VALUES[i % HEADER_VALUES.len()];

        let header = format!("{name}: {value}-{i}\r\n");

        headers.push(header);
    }

    headers
}

fn benchmark(c: &mut Criterion) {
    for &n in &[1_000, 10_000, 50_000] {
        let headers = generate_headers(n);

        c.bench_function(&format!("headers_{n}"), |b| {
            b.iter(|| {
                let mut rng = rand::rng();

                let mut shuffled = headers.clone();
                shuffled.shuffle(&mut rng);

                for header in shuffled.iter() {
                    black_box(parser::parse_header(Limits::default(), header).unwrap());
                }
            });
        });
    }
}

criterion_group!(bench, benchmark);
criterion_main!(bench);
