use crate::constants::ASPECT_RATIO;
use crate::constants::Z_FAR;
use crate::constants::Z_NEAR;

use crate::math::fov_rad;

pub(crate) struct Matrix(pub(crate) [[f32; 4]; 4]);

impl Matrix {
    // Once_cell ?
    pub(crate) fn projection_matrix() -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = ASPECT_RATIO * fov_rad();
        m[1][1] = fov_rad();
        m[2][2] = Z_FAR / (Z_FAR - Z_NEAR);
        m[3][2] = (-Z_FAR * Z_NEAR) / (Z_FAR - Z_NEAR);
        m[2][3] = 1.0;
        Matrix(m)
    }

    // Once_cell ?
    pub(crate) fn rotation_z(theta: f32) -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = f32::cos(theta);
        m[0][1] = f32::sin(theta);
        m[1][0] = -f32::sin(theta);
        m[1][1] = f32::cos(theta);
        m[2][2] = 1.0;
        m[3][3] = 1.0;
        Matrix(m)
    }

    // Once_cell ?
    pub(crate) fn rotation_x(theta: f32) -> Self {
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
