use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use physics_engine::simulation::world::*;

fn world_tick_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("World");

    for num_bodies in [1, 10, 100, 1000] {
        let world = World::new_populated(1920., 1080., 10., num_bodies);

        group.bench_function(BenchmarkId::new("tick", num_bodies), |b| {
            b.iter(|| world.clone().tick(Duration::new(1, 0)))
        });
    }
}

criterion_group!(benches, world_tick_benchmark);
criterion_main!(benches);
