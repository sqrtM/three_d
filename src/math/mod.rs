use crate::constants::FOV;

pub(crate) mod matrix;

fn fov_rad() -> f32 {
    1.0 / f32::tan(f32::to_radians(FOV * 0.5))
}
