use std::ops::{Add, Mul, Sub};

use crate::math::matrix::Matrix;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Point3d {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl Point3d {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self {
        Point3d { x, y, z }
    }

    pub(crate) fn dot_product(&self, rhs: Point3d) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub(crate) fn normalize(&self) -> Self {
        let normalization_factor = f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        Point3d {
            x: self.x / normalization_factor,
            y: self.y / normalization_factor,
            z: self.z / normalization_factor,
        }
    }
}

impl Add<Point3d> for Point3d {
    type Output = Self;

    fn add(self, rhs: Point3d) -> Self::Output {
        Point3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Point3d> for Point3d {
    type Output = Self;

    fn sub(self, rhs: Point3d) -> Self::Output {
        Point3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Point3d> for Point3d {
    type Output = Self;

    fn mul(self, rhs: Point3d) -> Self::Output {
        Point3d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Matrix> for &Point3d {
    type Output = Point3d;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let w =
            (self.x * rhs.0[0][3]) + (self.y * rhs.0[1][3]) + (self.z * rhs.0[2][3]) + rhs.0[3][3];
        if w != 0.0 {
            Point3d {
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
            Point3d {
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
