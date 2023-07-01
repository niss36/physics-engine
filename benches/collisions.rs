use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, Criterion,
};
use physics_engine::{
    body::*,
    simulation::collisions::{generate_contact_dynamic, generate_contact_static},
    vec2::*,
};

fn generate_contact_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collisions");

    circle_to_circle_colliding_benchmark(&mut group);
    circle_to_circle_not_colliding_benchmark(&mut group);
    line_to_circle_colliding_benchmark(&mut group);
    line_to_circle_not_colliding_benchmark(&mut group);
}

fn circle_to_circle_colliding_benchmark<M: Measurement>(group: &mut BenchmarkGroup<'_, M>) {
    let circle_1 = DynamicBody::Circle(Circle {
        body: BaseDynamicBody {
            position: ZERO,
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    let circle_2 = DynamicBody::Circle(Circle {
        body: BaseDynamicBody {
            position: UNIT_RIGHT,
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    group.bench_with_input(
        "circle to circle (colliding)",
        &(circle_1, circle_2),
        |b, (circle_1, circle_2)| b.iter(|| generate_contact_dynamic(circle_1, circle_2)),
    );
}

fn circle_to_circle_not_colliding_benchmark<M: Measurement>(group: &mut BenchmarkGroup<'_, M>) {
    let circle_1 = DynamicBody::Circle(Circle {
        body: BaseDynamicBody {
            position: ZERO,
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    let circle_2 = DynamicBody::Circle(Circle {
        body: BaseDynamicBody {
            position: Vec2D { x: 10., y: 10. },
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    group.bench_with_input(
        "circle to circle (not colliding)",
        &(circle_1, circle_2),
        |b, (circle_1, circle_2)| b.iter(|| generate_contact_dynamic(circle_1, circle_2)),
    );
}

fn line_to_circle_colliding_benchmark<M: Measurement>(group: &mut BenchmarkGroup<'_, M>) {
    let circle = DynamicBody::Circle(Circle {
        body: BaseDynamicBody {
            position: ZERO,
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    let line = StaticBody::Line(Line {
        normal: UNIT_RIGHT,
        origin_distance: 3.,
    });

    group.bench_with_input(
        "line to circle (colliding)",
        &(line, circle),
        |b, (line, circle)| b.iter(|| generate_contact_static(line, circle)),
    );
}

fn line_to_circle_not_colliding_benchmark<M: Measurement>(group: &mut BenchmarkGroup<'_, M>) {
    let circle = DynamicBody::Circle(Circle {
        body: BaseDynamicBody {
            position: ZERO,
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    let line = StaticBody::Line(Line {
        normal: UNIT_RIGHT,
        origin_distance: 10.,
    });

    group.bench_with_input(
        "line to circle (not colliding)",
        &(line, circle),
        |b, (line, circle)| b.iter(|| generate_contact_static(line, circle)),
    );
}

criterion_group!(collisions_benches, generate_contact_benchmark);
criterion_main!(collisions_benches);
