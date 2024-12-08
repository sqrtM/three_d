extern crate sdl2;
use crate::primatives::mesh::Mesh;
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use primatives::point3d::Point3d;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod constants;
mod math;
mod primatives;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let camera = Point3d::default();

    let window = video_subsystem
        .window("cube rotating", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let theta = 1.0 + 0.001 * timer_subsystem.ticks64() as f32;
        for tri in Mesh::unit_cube().0.iter() {
            let triangle = tri.rotate_z(theta).rotate_x(theta).translate();

            // If visible...
            if triangle.normal_vector().dot_product(triangle.a - camera) < 0. {
                triangle.project().scale().draw_filled(&mut canvas);
            }
        }

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
}
