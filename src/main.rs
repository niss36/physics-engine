use physics_engine::{rendering::*, simulation::world::World, vec2::Vec2D};

use macroquad::prelude::*;

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

const TIME_BETWEEN_TICKS: f32 = 10. / 1_000.;

fn generate_world() -> World {
    World::generate(screen_width() as f64, screen_height() as f64, 10., 500)
}

#[derive(Debug)]
struct IncrementalStatistics {
    n_samples: u64,
    average: f64,
}

impl IncrementalStatistics {
    fn new() -> Self {
        Self {
            n_samples: 0,
            average: 0.,
        }
    }

    fn add_measurement(&mut self, measurement: f64) {
        self.average += (measurement - self.average) / ((self.n_samples + 1) as f64);
        self.n_samples += 1;
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = generate_world();
    let mut accumulator = 0.;

    let mut tick_statistics = IncrementalStatistics::new();
    let mut render_statistics = IncrementalStatistics::new();

    loop {
        if is_key_released(KeyCode::R) {
            world = generate_world();
            accumulator = 0.;

            tick_statistics = IncrementalStatistics::new();
            render_statistics = IncrementalStatistics::new();
        }

        if is_key_released(KeyCode::Key1) {
            world.gravity = Vec2D { x: 0., y: -100. };
        }

        if is_key_released(KeyCode::Key2) {
            world.gravity = Vec2D { x: 100., y: -100. };
        }

        if is_key_released(KeyCode::Key3) {
            world.gravity = Vec2D { x: 100., y: 0. };
        }

        if is_key_released(KeyCode::Key4) {
            world.gravity = Vec2D { x: 100., y: 100. };
        }

        if is_key_released(KeyCode::Key5) {
            world.gravity = Vec2D { x: 0., y: 100. };
        }

        if is_key_released(KeyCode::Key6) {
            world.gravity = Vec2D { x: -100., y: 100. };
        }

        if is_key_released(KeyCode::Key7) {
            world.gravity = Vec2D { x: -100., y: 0. };
        }

        if is_key_released(KeyCode::Key8) {
            world.gravity = Vec2D { x: -100., y: -100. };
        }

        if is_key_released(KeyCode::Key0) {
            world.gravity = Vec2D { x: 0., y: 0. };
        }

        accumulator += get_frame_time();
        accumulator = accumulator.min(TIME_BETWEEN_TICKS * 5.);

        let mut ticks_per_frame = 0;

        while accumulator >= TIME_BETWEEN_TICKS && ticks_per_frame < 5 {
            accumulator -= TIME_BETWEEN_TICKS;

            let before_tick = get_time();
            world.tick(TIME_BETWEEN_TICKS as f64);
            let elapsed_tick = get_time() - before_tick;

            tick_statistics.add_measurement(elapsed_tick);

            ticks_per_frame += 1;
        }

        clear_background(WHITE);

        let before_render = get_time();
        render_world(&world);
        let elapsed_render = get_time() - before_render;

        render_statistics.add_measurement(elapsed_render);

        draw_text(format!("{} FPS", get_fps()).as_str(), 10., 10., 16., RED);
        draw_text(
            format!("{ticks_per_frame} ticks per frame").as_str(),
            10.,
            30.,
            16.,
            RED,
        );
        draw_text(
            format!("{:.3} ms tick", tick_statistics.average * 1_000.).as_str(),
            10.,
            50.,
            16.,
            RED,
        );
        draw_text(
            format!("{:.3} ms render", render_statistics.average * 1_000.).as_str(),
            10.,
            70.,
            16.,
            RED,
        );
        draw_text(
            format!("{} bodies", world.dynamic_bodies.len()).as_str(),
            10.,
            90.,
            16.,
            RED,
        );

        next_frame().await
    }
}
