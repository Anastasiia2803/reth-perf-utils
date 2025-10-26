use criterion::{criterion_group, criterion_main, Criterion, black_box};

fn bench_u256_add(c: &mut Criterion) {
    c.bench_function("u256_add", |b| {
        b.iter(|| {
            let mut x: u128 = 0;
            for i in 0..1000u128 {
                x = x.wrapping_add(i);
            }
            black_box(x)
        })
    });
}

criterion_group!(benches, bench_u256_add);
criterion_main!(benches);
