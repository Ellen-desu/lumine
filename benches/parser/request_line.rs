use criterion::{Criterion, criterion_group, criterion_main};
use lumine::parse_request_line_for_bench;
use rand::seq::SliceRandom;
use std::hint::black_box;

fn generate_request_lines(n: usize) -> Vec<String> {
    const METHODS: &[&str] = &["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"];

    const VERSIONS: &[&str] = &["HTTP/1.1", "HTTP/2"];

    const ROUTES: &[&str] = &[
        "/",
        "/users",
        "/users/profile",
        "/users/settings",
        "/posts",
        "/posts/trending",
        "/catalog/products",
        "/catalog/products/search",
        "/api/v1/auth/login",
        "/api/v1/auth/logout",
        "/api/v1/orders",
        "/assets/images/logo.png",
        "/assets/js/app.js",
        "/health",
        "/metrics",
    ];

    const USER_AGENTS: &[&str] = &["chrome", "firefox", "safari", "edge", "bot"];

    let mut lines = Vec::with_capacity(n);

    for i in 0..n {
        let method = METHODS[i % METHODS.len()];
        let version = VERSIONS[i % VERSIONS.len()];
        let route = ROUTES[i % ROUTES.len()];
        let agent = USER_AGENTS[i % USER_AGENTS.len()];

        let user_id = i * 17 + 42;
        let post_id = i * 31 + 99;

        let query = match i % 10 {
            0 => format!("?page={}&limit=20", i % 50),
            1 => "?search=rust+http+server&sort=desc".to_string(),
            2 => format!("?user_id={user_id}&include=posts,comments"),
            3 => format!("?cache_bust={:x}", i * 918273),
            4 => "?redirect=%2Fdashboard%3Ftab%3Dprofile".to_string(),
            5 => "?tag=networking&tag=rust&tag=async".to_string(),
            6 => format!("?session={:032x}", i.wrapping_mul(0xdeadbeef)),
            7 => "?emoji=%F0%9F%9A%80".to_string(),
            8 => format!("?utm_source={agent}&utm_campaign=summer_sale&utm_medium=banner"),
            _ => format!("?post_id={post_id}&expand[]=author&expand[]=stats"),
        };

        let path = match i % 8 {
            0 => format!("{route}/{user_id}"),
            1 => format!("{route}/{user_id}/edit"),
            2 => format!("{route}/{post_id}/comments"),
            3 => format!("{route}/archive/2025/12"),
            4 => format!("{route}/../../etc/passwd"),
            5 => format!("{route}/{}", "a".repeat(i % 128)),
            6 => format!("{route}/{}", "%2e%2e%2f".repeat(i % 8)),
            _ => route.to_string(),
        };

        lines.push(format!("{method} {path}{query} {version}"));
    }

    lines
}

fn benchmark(c: &mut Criterion) {
    for &n in &[1_000, 10_000, 50_000] {
        let lines = generate_request_lines(n);

        c.bench_function(&format!("request_line_{n}"), |b| {
            b.iter(|| {
                let mut rng = rand::rng();

                let mut shuffled = lines.clone();
                shuffled.shuffle(&mut rng);

                for line in shuffled.iter() {
                    black_box(parse_request_line_for_bench(line, 1024, 1024, 100).unwrap());
                }
            });
        });
    }
}

criterion_group!(bench, benchmark);
criterion_main!(bench);
