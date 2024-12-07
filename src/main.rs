extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::video::Window;
use std::ops::Mul;

const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 1000.0;
const FOV: f32 = 90.0;
const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 960;
const ASPECT_RATIO: f32 = SCREEN_HEIGHT as f32 / SCREEN_WIDTH as f32;
fn fov_rad() -> f32 {
    1.0 / f32::tan(f32::to_radians(FOV * 0.5))
}

#[derive(Debug)]
struct Vec3d {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3d {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3d { x, y, z }
    }
}

#[derive(Debug)]
struct Triangle {
    a: Vec3d,
    b: Vec3d,
    c: Vec3d,
}

impl Triangle {
    fn new(a: Vec3d, b: Vec3d, c: Vec3d) -> Self {
        Triangle { a, b, c }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<Window>) {
        canvas.set_draw_color(Color::MAGENTA);
        canvas
            .draw_fline((self.a.x, self.a.y), (self.b.x, self.b.y))
            .unwrap();
        canvas.set_draw_color(Color::GREEN);
        canvas
            .draw_fline((self.b.x, self.b.y), (self.c.x, self.c.y))
            .unwrap();
        canvas.set_draw_color(Color::CYAN);
        canvas
            .draw_fline((self.c.x, self.c.y), (self.a.x, self.a.y))
            .unwrap();
    }

    fn project(&self) -> Self {
        Self {
            a: &self.a * Matrix::projection_matrix(),
            b: &self.b * Matrix::projection_matrix(),
            c: &self.c * Matrix::projection_matrix(),
        }
    }

    fn translate(&self) -> Self {
        Self {
            a: Vec3d {
                x: self.a.x + 1.0,
                y: self.a.y,
                z: self.a.z + 2.0,
            },
            b: Vec3d {
                x: self.b.x + 1.0,
                y: self.b.y,
                z: self.b.z + 2.0,
            },
            c: Vec3d {
                x: self.c.x + 1.0,
                y: self.c.y,
                z: self.c.z + 2.0,
            },
        }
    }

    fn scale(&self) -> Self {
        Self {
            a: Vec3d {
                x: ((self.a.x + 1.0) * 0.3) * SCREEN_WIDTH as f32,
                y: ((self.a.y + 1.0) * 0.3) * SCREEN_HEIGHT as f32,
                z: self.a.z,
            },
            b: Vec3d {
                x: ((self.b.x + 1.0) * 0.3) * SCREEN_WIDTH as f32,
                y: ((self.b.y + 1.0) * 0.3) * SCREEN_HEIGHT as f32,
                z: self.b.z,
            },
            c: Vec3d {
                x: ((self.c.x + 1.0) * 0.3) * SCREEN_WIDTH as f32,
                y: ((self.c.y + 1.0) * 0.3) * SCREEN_HEIGHT as f32,
                z: self.c.z,
            },
        }
    }

    fn rotate_z(&self, theta: f32) -> Self {
        Self {
            a: &self.a * Matrix::rotation_z(theta),
            b: &self.b * Matrix::rotation_z(theta),
            c: &self.c * Matrix::rotation_z(theta),
        }
    }

    fn rotate_x(&self, theta: f32) -> Self {
        Self {
            a: &self.a * Matrix::rotation_x(theta),
            b: &self.b * Matrix::rotation_x(theta),
            c: &self.c * Matrix::rotation_x(theta),
        }
    }
}

struct Mesh(Vec<Triangle>);

impl Mesh {
    fn unit_cube() -> Self {
        Mesh(vec![
            // SOUTH FACE
            Triangle::new(
                Vec3d::new(0.0, 0.0, 0.0),
                Vec3d::new(0.0, 1.0, 0.0),
                Vec3d::new(1.0, 1.0, 0.0),
            ),
            Triangle::new(
                Vec3d::new(0.0, 0.0, 0.0),
                Vec3d::new(1.0, 1.0, 0.0),
                Vec3d::new(1.0, 0.0, 0.0),
            ),
            // EAST FACE
            Triangle::new(
                Vec3d::new(1.0, 0.0, 0.0),
                Vec3d::new(1.0, 1.0, 0.0),
                Vec3d::new(1.0, 1.0, 1.0),
            ),
            Triangle::new(
                Vec3d::new(1.0, 0.0, 0.0),
                Vec3d::new(1.0, 1.0, 1.0),
                Vec3d::new(1.0, 0.0, 1.0),
            ),
            // NORTH FACE
            Triangle::new(
                Vec3d::new(1.0, 0.0, 1.0),
                Vec3d::new(1.0, 1.0, 1.0),
                Vec3d::new(0.0, 1.0, 1.0),
            ),
            Triangle::new(
                Vec3d::new(1.0, 0.0, 1.0),
                Vec3d::new(0.0, 1.0, 1.0),
                Vec3d::new(0.0, 0.0, 1.0),
            ),
            // WEST FACE
            Triangle::new(
                Vec3d::new(0.0, 0.0, 1.0),
                Vec3d::new(0.0, 1.0, 1.0),
                Vec3d::new(0.0, 1.0, 0.0),
            ),
            Triangle::new(
                Vec3d::new(0.0, 0.0, 1.0),
                Vec3d::new(0.0, 1.0, 0.0),
                Vec3d::new(0.0, 0.0, 0.0),
            ),
            // TOP FACE
            Triangle::new(
                Vec3d::new(0.0, 1.0, 0.0),
                Vec3d::new(0.0, 1.0, 1.0),
                Vec3d::new(1.0, 1.0, 1.0),
            ),
            Triangle::new(
                Vec3d::new(0.0, 1.0, 0.0),
                Vec3d::new(1.0, 1.0, 1.0),
                Vec3d::new(1.0, 1.0, 0.0),
            ),
            // BOTTOM FACE
            Triangle::new(
                Vec3d::new(1.0, 0.0, 1.0),
                Vec3d::new(0.0, 0.0, 1.0),
                Vec3d::new(0.0, 0.0, 0.0),
            ),
            Triangle::new(
                Vec3d::new(1.0, 0.0, 1.0),
                Vec3d::new(0.0, 0.0, 0.0),
                Vec3d::new(1.0, 0.0, 0.0),
            ),
        ])
    }
}

struct Matrix([[f32; 4]; 4]);

impl Matrix {
    fn projection_matrix() -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = ASPECT_RATIO * fov_rad();
        m[1][1] = fov_rad();
        m[2][2] = Z_FAR / (Z_FAR - Z_NEAR);
        m[3][2] = (-Z_FAR * Z_NEAR) / (Z_FAR - Z_NEAR);
        m[2][3] = 1.0;
        Matrix(m)
    }

    fn rotation_z(theta: f32) -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = f32::cos(theta);
        m[0][1] = f32::sin(theta);
        m[1][0] = -f32::sin(theta);
        m[1][1] = f32::cos(theta);
        m[2][2] = 1.0;
        m[3][3] = 1.0;
        Matrix(m)
    }

    fn rotation_x(theta: f32) -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = 1.0;
        m[1][1] = f32::cos(theta * 0.5);
        m[1][2] = f32::sin(theta * 0.5);
        m[2][1] = -f32::sin(theta * 0.5);
        m[2][2] = f32::cos(theta * 0.5);
        m[3][3] = 1.0;
        Matrix(m)
    }
}

impl Mul<Matrix> for &Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let w =
            (self.x * rhs.0[0][3]) + (self.y * rhs.0[1][3]) + (self.z * rhs.0[2][3]) + rhs.0[3][3];
        if w != 0.0 {
            Vec3d {
                x: ((self.x * rhs.0[0][0])
                    + (self.y * rhs.0[1][0])
                    + (self.z * rhs.0[2][0])
                    + rhs.0[3][0])
                    / w,
                y: ((self.x * rhs.0[0][1])
                    + (self.y * rhs.0[1][1])
                    + (self.z * rhs.0[2][1])
                    + rhs.0[3][1])
                    / w,
                z: ((self.x * rhs.0[0][2])
                    + (self.y * rhs.0[1][2])
                    + (self.z * rhs.0[2][2])
                    + rhs.0[3][2])
                    / w,
            }
        } else {
            Vec3d {
                x: ((self.x * rhs.0[0][0])
                    + (self.y * rhs.0[1][0])
                    + (self.z * rhs.0[2][0])
                    + rhs.0[3][0]),
                y: ((self.x * rhs.0[0][1])
                    + (self.y * rhs.0[1][1])
                    + (self.z * rhs.0[2][1])
                    + rhs.0[3][1]),
                z: ((self.x * rhs.0[0][2])
                    + (self.y * rhs.0[1][2])
                    + (self.z * rhs.0[2][2])
                    + rhs.0[3][2]),
            }
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

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
            tri.rotate_z(theta)
                .rotate_x(theta)
                .translate()
                .project()
                .scale()
                .draw(&mut canvas);
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
