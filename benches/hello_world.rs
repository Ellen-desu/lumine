use criterion::{Criterion, criterion_group, criterion_main};
use lumine::Lumine;
use std::{hint::black_box, net::TcpListener};

#[allow(clippy::unit_arg)]
fn benchmark(c: &mut Criterion) {
    let app = Lumine::builder().route("/", |_| "Hello World").build();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let _rx = app.serve(listener);

    c.bench_function("hello_world", |b| {
        b.iter(|| black_box(ureq::get("http://127.0.0.1:8080").call().unwrap()))
    });
}

criterion_group!(bench, benchmark);
criterion_main!(bench);
