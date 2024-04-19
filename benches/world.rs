use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use physics_engine::{simulation::world::*, vec2::ZERO};

fn world_tick_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("World");

    for num_bodies in [1, 10, 100, 1000, 10000] {
        let world = World::generate(1920., 1080., 10., num_bodies, ZERO);

        group.bench_function(BenchmarkId::new("tick", num_bodies), |b| {
            b.iter(|| world.clone().tick(1.))
        });
    }
}

criterion_group!(world_benches, world_tick_benchmark);
criterion_main!(world_benches);
