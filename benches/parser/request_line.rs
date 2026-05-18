use criterion::{Criterion, criterion_group, criterion_main};
use lumine::parse_request_line_for_bench;
use rand::seq::SliceRandom;
use std::hint::black_box;

fn generate_request_line(n: usize) -> Vec<String> {
    let mut lines = Vec::with_capacity(n);

    for i in 0..n {
        let method = match i % 5 {
            1 => "GET",
            2 => "POST",
            3 => "PATCH",
            4 => "PUT",
            _ => "DELETE",
        };

        let url = match i % 4 {
            1 => "/users",
            2 => "/posts",
            3 => "/catalogs",
            _ => "/",
        };

        let version = match i % 3 {
            1 => "HTTP/1.1",
            2 => "HTTP/2",
            _ => "HTTP/3",
        };

        lines.push(format!("{method} {url} {version}"));
    }
    lines
}

fn benchmark(c: &mut Criterion) {
    for &n in &[1_000, 10_000, 50_000] {
        let lines = generate_request_line(n);

        c.bench_function(&format!("request_line_{n}"), |b| {
            b.iter(|| {
                let mut rng = rand::rng();

                let mut shuffled = lines.clone();
                shuffled.shuffle(&mut rng);

                for line in shuffled.iter() {
                    black_box(parse_request_line_for_bench(line).unwrap());
                }
            });
        });
    }
}

criterion_group!(bench, benchmark);
criterion_main!(bench);
