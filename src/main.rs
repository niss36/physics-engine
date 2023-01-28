mod body;
mod rendering;
mod simulation;
mod vec2;

use rendering::*;
use simulation::world::World;

use macroquad::prelude::*;
use std::time::Instant;

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

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new_populated(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64, 10.);
    let mut instant = Instant::now();

    loop {
        clear_background(WHITE);

        render_world(&world);

        world.tick(instant.elapsed());
        instant = Instant::now();

        next_frame().await
    }
}