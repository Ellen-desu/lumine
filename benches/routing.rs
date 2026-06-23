use criterion::{Criterion, criterion_group, criterion_main};
use lumine::{application::states::Ready, prelude::*};
use rand::seq::SliceRandom;
use std::{hint::black_box, str::FromStr};

fn build_app() -> Lumine<Ready> {
    let mut builder = Lumine::builder();

    let routes = [
        "/users/:userid",
        "/stores/:storeid/products/:productid",
        "/categories/:categoryid/subcategories/:subcategoryid",
        "/companies/:companyid/employees/:employeeid/email",
    ];

    for r in routes {
        builder = builder.route(r, async |_| {});
    }

    builder.build()
}

fn generate_routes(n: usize) -> Vec<Uri> {
    let mut routes = Vec::with_capacity(n);

    for i in 0..n {
        let path = match i % 5 {
            1 => format!("/users/{i}"),
            2 => format!("/stores/{i}/products/{i}"),
            3 => format!("/categories/{i}/subcategories/{i}"),
            4 => format!("/companies/{i}/employees/{i}/email"),
            _ => "/nothing".into(),
        };

        routes.push(Uri::from_str(&path).unwrap());
    }

    routes
}

fn benchmark(c: &mut Criterion) {
    let app = build_app();

    for &n in &[1_000, 10_000, 50_000] {
        let routes = generate_routes(n);

        c.bench_function(&format!("route_matching_{n}"), |b| {
            b.iter(|| {
                let mut rng = rand::rng();

                let mut shuffled = routes.clone();
                shuffled.shuffle(&mut rng);

                for r in shuffled.iter() {
                    black_box(app.get_route(r));
                }
            });
        });
    }
}

criterion_group!(bench, benchmark);
criterion_main!(bench);
