use crate::primatives::{point3d::Point3d, triangle::Triangle};

pub(crate) struct Mesh(pub(crate) Vec<Triangle>);

impl Mesh {
    pub(crate) fn unit_cube() -> Self {
        Mesh(vec![
            // SOUTH FACE
            Triangle::new(
                Point3d::new(0.0, 0.0, 0.0),
                Point3d::new(0.0, 1.0, 0.0),
                Point3d::new(1.0, 1.0, 0.0),
            ),
            Triangle::new(
                Point3d::new(0.0, 0.0, 0.0),
                Point3d::new(1.0, 1.0, 0.0),
                Point3d::new(1.0, 0.0, 0.0),
            ),
            // EAST FACE
            Triangle::new(
                Point3d::new(1.0, 0.0, 0.0),
                Point3d::new(1.0, 1.0, 0.0),
                Point3d::new(1.0, 1.0, 1.0),
            ),
            Triangle::new(
                Point3d::new(1.0, 0.0, 0.0),
                Point3d::new(1.0, 1.0, 1.0),
                Point3d::new(1.0, 0.0, 1.0),
            ),
            // NORTH FACE
            Triangle::new(
                Point3d::new(1.0, 0.0, 1.0),
                Point3d::new(1.0, 1.0, 1.0),
                Point3d::new(0.0, 1.0, 1.0),
            ),
            Triangle::new(
                Point3d::new(1.0, 0.0, 1.0),
                Point3d::new(0.0, 1.0, 1.0),
                Point3d::new(0.0, 0.0, 1.0),
            ),
            // WEST FACE
            Triangle::new(
                Point3d::new(0.0, 0.0, 1.0),
                Point3d::new(0.0, 1.0, 1.0),
                Point3d::new(0.0, 1.0, 0.0),
            ),
            Triangle::new(
                Point3d::new(0.0, 0.0, 1.0),
                Point3d::new(0.0, 1.0, 0.0),
                Point3d::new(0.0, 0.0, 0.0),
            ),
            // TOP FACE
            Triangle::new(
                Point3d::new(0.0, 1.0, 0.0),
                Point3d::new(0.0, 1.0, 1.0),
                Point3d::new(1.0, 1.0, 1.0),
            ),
            Triangle::new(
                Point3d::new(0.0, 1.0, 0.0),
                Point3d::new(1.0, 1.0, 1.0),
                Point3d::new(1.0, 1.0, 0.0),
            ),
            // BOTTOM FACE
            Triangle::new(
                Point3d::new(1.0, 0.0, 1.0),
                Point3d::new(0.0, 0.0, 1.0),
                Point3d::new(0.0, 0.0, 0.0),
            ),
            Triangle::new(
                Point3d::new(1.0, 0.0, 1.0),
                Point3d::new(0.0, 0.0, 0.0),
                Point3d::new(1.0, 0.0, 0.0),
            ),
        ])
    }
}
