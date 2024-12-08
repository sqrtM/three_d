use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    math::matrix::Matrix,
};
use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, video::Window};

use crate::Point3d;

#[derive(Debug)]
pub(crate) struct Triangle {
    pub(crate) a: Point3d,
    pub(crate) b: Point3d,
    pub(crate) c: Point3d,
}

impl Triangle {
    pub(crate) fn new(a: Point3d, b: Point3d, c: Point3d) -> Self {
        Triangle { a, b, c }
    }

    pub(crate) fn draw_wireframe(&self, canvas: &mut sdl2::render::Canvas<Window>) {
        canvas.set_draw_color(Color::MAGENTA);
        canvas
            .draw_line(
                (self.a.x as i32, self.a.y as i32),
                (self.b.x as i32, self.b.y as i32),
            )
            .unwrap();
        canvas.set_draw_color(Color::GREEN);
        canvas
            .draw_line(
                (self.b.x as i32, self.b.y as i32),
                (self.c.x as i32, self.c.y as i32),
            )
            .unwrap();
        canvas.set_draw_color(Color::CYAN);
        canvas
            .draw_line(
                (self.c.x as i32, self.c.y as i32),
                (self.a.x as i32, self.a.y as i32),
            )
            .unwrap();
    }

    pub(crate) fn draw_filled(&self, canvas: &mut ::sdl2::render::Canvas<Window>, alpha: u8) {
        canvas
            .filled_trigon(
                self.a.x as i16,
                self.a.y as i16,
                self.b.x as i16,
                self.b.y as i16,
                self.c.x as i16,
                self.c.y as i16,
                Color::RGBA(255, 255, 0, alpha),
            )
            .unwrap();
    }

    pub(crate) fn project(&self) -> Self {
        Self {
            a: &self.a * Matrix::projection_matrix(),
            b: &self.b * Matrix::projection_matrix(),
            c: &self.c * Matrix::projection_matrix(),
        }
    }

    pub(crate) fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        Self {
            a: Point3d {
                x: self.a.x + x,
                y: self.a.y + y,
                z: self.a.z + z,
            },
            b: Point3d {
                x: self.b.x + x,
                y: self.b.y + y,
                z: self.b.z + z,
            },
            c: Point3d {
                x: self.c.x + x,
                y: self.c.y + y,
                z: self.c.z + z,
            },
        }
    }

    pub(crate) fn scale(&self) -> Self {
        Self {
            a: Point3d {
                x: ((self.a.x + 1.0) * 0.3) * SCREEN_WIDTH as f32,
                y: ((self.a.y + 1.0) * 0.3) * SCREEN_HEIGHT as f32,
                z: self.a.z,
            },
            b: Point3d {
                x: ((self.b.x + 1.0) * 0.3) * SCREEN_WIDTH as f32,
                y: ((self.b.y + 1.0) * 0.3) * SCREEN_HEIGHT as f32,
                z: self.b.z,
            },
            c: Point3d {
                x: ((self.c.x + 1.0) * 0.3) * SCREEN_WIDTH as f32,
                y: ((self.c.y + 1.0) * 0.3) * SCREEN_HEIGHT as f32,
                z: self.c.z,
            },
        }
    }

    pub(crate) fn normal_vector(&self) -> Point3d {
        // Once_cell ?
        let line_a = Point3d {
            x: self.b.x - self.a.x,
            y: self.b.y - self.a.y,
            z: self.b.z - self.a.z,
        };
        let line_b = Point3d {
            x: self.c.x - self.a.x,
            y: self.c.y - self.a.y,
            z: self.c.z - self.a.z,
        };
        // Normalize cross product
        Point3d {
            x: (line_a.y * line_b.z) - (line_a.z * line_b.y),
            y: (line_a.z * line_b.x) - (line_a.x * line_b.z),
            z: (line_a.x * line_b.y) - (line_a.y * line_b.x),
        }
        .normalize()
    }

    pub(crate) fn rotate_z(&self, theta: f32) -> Self {
        Self {
            a: &self.a * Matrix::rotation_z(theta),
            b: &self.b * Matrix::rotation_z(theta),
            c: &self.c * Matrix::rotation_z(theta),
        }
    }

    pub(crate) fn rotate_x(&self, theta: f32) -> Self {
        Self {
            a: &self.a * Matrix::rotation_x(theta),
            b: &self.b * Matrix::rotation_x(theta),
            c: &self.c * Matrix::rotation_x(theta),
        }
    }
}
