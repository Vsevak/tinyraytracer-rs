use std::io::Error;

use crate::geometry::GVec;
use geometry::Vec3f;
use render::{render, draw, Light};
use sphere::{Sphere, Material};

pub mod geometry;
pub mod render;
pub mod sphere;

fn main() -> Result<(), Error> {
    let ivory = Material{diffuse_color: Vec3f::new(0.4, 0.4, 0.3)};
    let red_rubber = Material{diffuse_color: Vec3f::new(0.3, 0.1, 0.1)};

    let spheres = vec![
        Sphere::new(Vec3f::new(-3.0, 0.0, -16.0), 2.0, ivory),
        Sphere::new(Vec3f::new(-1.0, -1.5, -12.0), 2.0, red_rubber),
        Sphere::new(Vec3f::new(1.5, -0.5, -18.0), 3.0, red_rubber),
        Sphere::new(Vec3f::new(7.0, 5.0, -18.0), 4.0, ivory)
    ];

    let lights = vec![
        Light::new(Vec3f::new(-20.0, 20.0,  20.0), 1.5)
    ];
    draw(&render(spheres, lights))
}
