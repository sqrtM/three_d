use std::fmt::Error;

use crate::{
    primatives::{point3d::Point3d, triangle::Triangle},
    utils::parse_file_line_by_line,
};

pub(crate) struct Mesh(pub(crate) Vec<Triangle>);

impl Mesh {
    pub(crate) fn load_obj(path: &str) -> Result<Self, Error> {
        let mut points: Vec<Point3d> = vec![];
        let mut triangles: Vec<Triangle> = vec![];
        parse_file_line_by_line(path, |mut line| {
            if let Some(first_char) = &line.chars().next() {
                match first_char {
                    'v' => {
                        // HACK !!!
                        if line.starts_with("vt") {
                            println!("Line begins with vt. Skipping...");
                            return;
                        }
                        let inputs = line
                            .split_off(1)
                            .split_whitespace()
                            .map(|string| string.parse::<f32>().unwrap())
                            .collect::<Vec<f32>>();
                        points.push(Point3d::new(inputs[0], inputs[1], inputs[2]));
                    }
                    'f' => {
                        let inputs = line
                            .split_off(1)
                            .split_whitespace()
                            .map(|string| string.parse::<i16>().unwrap())
                            .collect::<Vec<i16>>();
                        match inputs.len() {
                            0..=2 => {
                                panic!(
                                    "Length of {:?} vertices used for a face. Panicking...",
                                    inputs.len()
                                )
                            }
                            3 => triangles.push(Triangle::new(
                                points[(inputs[0] - 1) as usize],
                                points[(inputs[1] - 1) as usize],
                                points[(inputs[2] - 1) as usize],
                            )),
                            _ => {
                                // First, take the first three
                                triangles.push(Triangle::new(
                                    points[(inputs[0] - 1) as usize],
                                    points[(inputs[1] - 1) as usize],
                                    points[(inputs[2] - 1) as usize],
                                ));
                                let mut i = 3;
                                while i < inputs.len() {
                                    triangles.push(Triangle::new(
                                        points[(inputs[i - 3] - 1) as usize],
                                        points[(inputs[i - 1] - 1) as usize],
                                        points[(inputs[i] - 1) as usize],
                                    ));
                                    i += 1;
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Line begins with {:?}. Skipping...", first_char)
                    }
                }
            } else {
                println!("Line is empty. Skipping...");
            }
        })
        .unwrap();
        Ok(Mesh(triangles))
    }

    /*
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
    */
}
