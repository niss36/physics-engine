use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, Criterion,
};
use physics_engine::{body::*, simulation::collisions::generate_contact, vec2::*};

fn generate_contact_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collisions");

    circle_to_circle_colliding_benchmark(&mut group);
    circle_to_circle_not_colliding_benchmark(&mut group);
    circle_to_line_colliding_benchmark(&mut group);
    circle_to_line_not_colliding_benchmark(&mut group);
}

fn circle_to_circle_colliding_benchmark<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    let circle_1 = Body::Circle(Circle {
        body: BaseBody {
            position: ZERO,
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    let circle_2 = Body::Circle(Circle {
        body: BaseBody {
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
        |b, (circle_1, circle_2)| b.iter(|| generate_contact(&circle_1, &circle_2)),
    );
}

fn circle_to_circle_not_colliding_benchmark<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    let circle_1 = Body::Circle(Circle {
        body: BaseBody {
            position: ZERO,
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    let circle_2 = Body::Circle(Circle {
        body: BaseBody {
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
        |b, (circle_1, circle_2)| b.iter(|| generate_contact(&circle_1, &circle_2)),
    );
}

fn circle_to_line_colliding_benchmark<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    let circle = Body::Circle(Circle {
        body: BaseBody {
            position: ZERO,
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    let line = Body::Line(Line::new(UNIT_RIGHT, 3.));

    for (input, id) in [
        (&(circle, line), "circle to line (colliding)"),
        (&(line, circle), "line to circle (colliding)"),
    ] {
        group.bench_with_input(id, input, |b, (body_1, body_2)| {
            b.iter(|| generate_contact(body_1, body_2))
        });
    }
}

fn circle_to_line_not_colliding_benchmark<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    let circle = Body::Circle(Circle {
        body: BaseBody {
            position: ZERO,
            velocity: ZERO,
            coefficient_of_restitution: 0.,
            inverse_mass: 1.,
        },
        radius: 5.,
    });

    let line = Body::Line(Line::new(UNIT_RIGHT, 10.));

    for (input, id) in [
        (&(circle, line), "circle to line (not colliding)"),
        (&(line, circle), "line to circle (not colliding)"),
    ] {
        group.bench_with_input(id, input, |b, (body_1, body_2)| {
            b.iter(|| generate_contact(body_1, body_2))
        });
    }
}

criterion_group!(collisions_benches, generate_contact_benchmark);
criterion_main!(collisions_benches);
