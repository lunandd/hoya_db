use criterion::{black_box, criterion_group, criterion_main, Criterion};
use db::{DBTypes, Database};
use rayon::prelude::*;
use std::sync::RwLock;
use std::time::Instant;

pub fn db_benchmark(c: &mut Criterion) {
    let db = RwLock::new(Database::new());

    c.bench_function("put", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            (0..iters).collect::<Vec<u64>>().par_iter().for_each(|i| {
                black_box(db.write().unwrap().put(
                    i.to_string(),
                    DBTypes::Number(i.to_string().parse::<isize>().unwrap()),
                ));
            });
            start.elapsed()
        })
    });

    println!("{:?}", db);

    c.bench_function("remove", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            (0..iters).collect::<Vec<u64>>().par_iter().for_each(|i| {
                black_box(db.write().unwrap().remove(&i.to_string()));
            });
            start.elapsed()
        })
    });

    println!("{:?}", db);

    c.bench_function("get", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for i in 0..iters {
                black_box(db.read().unwrap().get(&i.to_string()));
            }
            start.elapsed()
        })
    });

    println!("{:?}", db);

    c.bench_function("exists", |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for i in 0..iters {
                black_box(db.read().unwrap().exists(&i.to_string()));
            }
            start.elapsed()
        })
    });

    println!("{:?}", db);
}

criterion_group!(benches, db_benchmark);
criterion_main!(benches);
