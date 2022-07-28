use std::io::Error;

use crate::geometry::GVec;
use render::{render, draw, Vec3f};
use sphere::Sphere;

pub mod geometry;
pub mod render;
pub mod sphere;

fn main() -> Result<(), Error> {
    let sphere = Sphere{
        center: Vec3f::from([-3.0, 0.0, -16.0]),
        radius: 2.0
    };
    draw(&render(sphere))
}
