use std::{
    hint::black_box,
    io::{BufReader, Cursor},
};

use criterion::{Criterion, criterion_group, criterion_main};
use lumine::parse_body_for_bench;

fn benchmark(c: &mut Criterion) {
    let sizes = [
        128,               // 128B
        1024,              // 1KB
        16 * 1024,         // 16KB
        64 * 1024,         // 64KB
        1024 * 1024,       // 1MB
        1024 * 1024 * 10,  // 10MB
        1024 * 1024 * 100, // 100MB
    ];

    for size in sizes {
        let data = vec![b'a'; size];

        c.bench_function(&format!("body_{}b", size), |b| {
            b.iter(|| {
                let cursor = Cursor::new(black_box(&data));

                let mut reader = BufReader::new(cursor);

                black_box(parse_body_for_bench(size, &mut reader).unwrap())
            });
        });
    }
}

criterion_group!(bench, benchmark);
criterion_main!(bench);
