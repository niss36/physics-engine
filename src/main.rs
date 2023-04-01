use physics_engine::{rendering::*, simulation::world::World};

use macroquad::prelude::*;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: i32 = 1920;
const WINDOW_HEIGHT: i32 = 1080;

fn window_conf() -> Conf {
    Conf {
        window_title: "Physics Engine".into(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

const TIME_BETWEEN_TICKS: Duration = Duration::from_millis(10);

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::generate(screen_width() as f64, screen_height() as f64, 10., 100);
    let mut last_frame = Instant::now();
    let mut accumulator = Duration::default();

    let mut average_tick = Duration::default();
    let mut n_tick = 0;

    loop {
        accumulator += last_frame.elapsed();
        last_frame = Instant::now();

        let mut ticks_per_frame = 0;

        while accumulator >= TIME_BETWEEN_TICKS && ticks_per_frame < 5 {
            accumulator -= TIME_BETWEEN_TICKS;

            let before_tick = Instant::now();
            world.tick(TIME_BETWEEN_TICKS);
            let elapsed_tick = before_tick.elapsed();

            average_tick = (elapsed_tick + n_tick * average_tick) / (n_tick + 1);
            n_tick += 1;

            ticks_per_frame += 1;
        }

        clear_background(WHITE);

        render_world(&world);
        draw_text(format!("{} FPS", get_fps()).as_str(), 10., 10., 16., RED);
        draw_text(
            format!("{} ticks per frame", ticks_per_frame).as_str(),
            10.,
            30.,
            16.,
            RED,
        );
        draw_text(
            format!("{} ns tick", average_tick.as_nanos()).as_str(),
            10.,
            50.,
            16.,
            RED,
        );

        next_frame().await
    }
}
