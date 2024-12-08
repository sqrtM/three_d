extern crate sdl2;

use crate::primatives::mesh::Mesh;
use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use primatives::point3d::Point3d;
use primatives::triangle::Triangle;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

mod constants;
mod math;
mod primatives;
mod utils;

struct AppState {
    dragging: bool,
    fill_state: FillState,
    transform: Point3d,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            dragging: false,
            fill_state: FillState::Wireframe,
            transform: Point3d::default(),
        }
    }
}

enum FillState {
    Wireframe,
    Filled,
    Both,
}

impl AppState {
    fn increment_fill_state(&mut self) {
        match self.fill_state {
            FillState::Wireframe => self.fill_state = FillState::Filled,
            FillState::Filled => self.fill_state = FillState::Both,
            FillState::Both => self.fill_state = FillState::Wireframe,
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let camera = Point3d::default();
    let directional_light = Point3d {
        x: 0.,
        y: 0.,
        z: -1.,
    };

    let mut app_state = AppState::default();

    let window = video_subsystem
        .window("some stuff rotating", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mesh = Mesh::load_obj("./models/cow.obj").unwrap().0;
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let theta = 1.0 + 0.001 * timer_subsystem.ticks64() as f32;

        let mut tris_to_draw: Vec<Triangle> = vec![];
        for tri in mesh.iter() {
            let triangle = tri.rotate_z(theta).rotate_x(theta).translate(
                app_state.transform.x,
                app_state.transform.y,
                app_state.transform.z,
            );

            // If visible...
            if triangle.normal_vector().dot_product(triangle.a - camera) < 0. {
                tris_to_draw.push(triangle.project().scale());
            }
        }

        tris_to_draw.sort_unstable_by(|a, b| {
            let avg_a = (a.a.z + a.b.z + a.c.z) / 3.0;
            let avg_b = (b.a.z + b.b.z + b.c.z) / 3.0;
            avg_a
                .partial_cmp(&avg_b)
                .unwrap_or(std::cmp::Ordering::Equal)
                .reverse()
        });
        for tri in tris_to_draw.into_iter() {
            match app_state.fill_state {
                FillState::Wireframe => tri.draw_wireframe(&mut canvas),
                FillState::Filled => {
                    let alpha = (tri
                        .normal_vector()
                        .dot_product(directional_light.normalize())
                        * 255.) as u8;
                    tri.draw_filled(&mut canvas, alpha)
                }
                FillState::Both => {
                    let alpha = (tri
                        .normal_vector()
                        .dot_product(directional_light.normalize())
                        * 255.) as u8;
                    tri.draw_filled(&mut canvas, alpha);
                    tri.draw_wireframe(&mut canvas)
                }
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
                Event::KeyDown {
                    keycode: Some(Keycode::TAB),
                    ..
                } => {
                    app_state.increment_fill_state();
                }
                Event::MouseWheel { y, .. } => app_state.transform.z += y as f32 / 2.,
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Middle {
                        app_state.dragging = true;
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Middle {
                        app_state.dragging = false;
                    }
                }
                Event::MouseMotion { xrel, yrel, .. } => {
                    if app_state.dragging {
                        app_state.transform.x += xrel as f32 / 15.;
                        app_state.transform.y += yrel as f32 / 15.;
                    }
                }
                _ => {}
            }
        }
    }
}
