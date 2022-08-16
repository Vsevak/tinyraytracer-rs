use std::borrow::Borrow;
use std::f32::consts::PI;
use std::io::Error;
use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, ImageData};

use render::{View, Scene, RenderType};

use crate::geometry::{Vec3f, Vec4f};
use crate::render::{Light};
use crate::sphere::{Sphere, Material};

pub mod geometry;
pub mod render;
pub mod sphere;
pub mod march;
pub mod noise;

#[wasm_bindgen]
pub fn draw(
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    tick: i32
) -> Result<(), JsValue> {
    let ivory = Material {
        diffuse_color: Vec3f::new(0.4, 0.4, 0.3),
        albedo: Vec4f::new(0.6, 0.3, 0.1, 0.0),
        specular_exp: 50.,
        refractive_index: 1.0
    };
    let red_rubber = Material {
        diffuse_color: Vec3f::new(0.3, 0.1, 0.1),
        albedo: Vec4f::new(0.9,  0.1, 0.0, 0.0),
        specular_exp: 10.,
        refractive_index: 1.0
    };
    let mirror = Material {
        diffuse_color: Vec3f::one(),
        albedo: Vec4f::new(0.2, 10.0, 0.8, 0.0),
        specular_exp: 1425.0,
        refractive_index: 1.0
    };
    let glass = Material {
        albedo: Vec4f::new(0.0,  0.5, 0.1, 0.8),
        diffuse_color: Vec3f::new(0.6, 0.7, 0.8),
        refractive_index: 2.5,
        specular_exp: 125.0
    };

    let spheres = vec![
        Sphere::new(Vec3f::new(-3.0, 0.0, -16.0), 2.0, &ivory),
        Sphere::new(Vec3f::new(-1.0, -1.5, -12.0), 2.0, &glass),
        Sphere::new(Vec3f::new(1.5, -0.5, -18.0), 3.0, &red_rubber),
        Sphere::new(Vec3f::new(7.0, 5.0, -18.0), 4.0, &mirror)
    ];

    let lights = vec![
        Light::new(Vec3f::new(-20.0, 20.0,  20.0), 1.5),
        Light::new(Vec3f::new( 30.0, 50.0, -25.0), 1.8),
        Light::new(Vec3f::new( 30.0, 20.0,  30.0), 1.7),
    ];
    let scene = Scene::new(spheres, lights);
    let small = View::new(width as usize, height as usize,PI / 3.0);

    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");
    
    // let canvas = document
    // .create_element("canvas")?
    // .dyn_into::<web_sys::HtmlCanvasElement>()?;
    // body.append_child(&canvas)?;
    // canvas.set_width(width);
    // canvas.set_height(height);
    
    // let ctx = canvas
    // .get_context("2d")?
    // .unwrap()
    // .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let data = small.render(RenderType::RayTrace(&scene), tick);

    let img = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut data.as_u8()), width, height).unwrap();
    ctx.put_image_data(&img, 0.0, 0.0)
}
