use std::f32::consts::PI;
use std::fs::{File};
use std::io::{prelude::*, Error, BufWriter};
use std::path::Path;

use crate::geometry::Vec3f;
use crate::sphere::{Sphere, Material};

pub struct Frame(Vec<Vec3f>, usize, usize);

pub fn render(spheres: Vec<Sphere>, lights: Vec<Light>) -> Frame {
    let width = 1024;
    let height = 768;
    let fov = PI / 3.0;
    let mut framebuffer: Vec<Vec3f> = Vec::new();
    framebuffer.resize(width*height, Vec3f::from([0.0, 0.0, 0.0]));
    for j in 0..height {
        for i in 0..width {
            let fheight = height as f32;
            let fwidth = width as f32;
            let fi = i as f32;
            let fj = j as f32;
            let x =  (fi + 0.5) - fwidth / 2.0;
            let y = -(fj + 0.5) + fheight / 2.0;
            let z = -fheight/(2.0*f32::tan(fov/2.0));
            let dir = Vec3f::from([x, y, z]).normalize();
            let pixel = cast_ray(Vec3f::from([0.0, 0.0, 0.0]), dir, &spheres, &lights);
            let max = pixel[0].max(pixel[1].max(pixel[2]));
            framebuffer[i+j*width] =  if max > 1.0 {
                pixel * (1.0/max)
            } else {
                pixel
            };
        }
    }
    Frame(framebuffer, width, height)
}

pub fn draw(frame: &Frame) -> Result<(), Error> {
    let (framebuffer, width, height) = (&frame.0, frame.1, frame.2);
    let path = Path::new("./out.ppm");
    let mut file = File::create(&path)?;
    write!(file, "P6\n{} {}\n255\n", width, height)?;
    let mut file_buff = BufWriter::new(file);
    for point in framebuffer {
        for i in 0..3 {
            file_buff.write(
                &[(255.0f32 * 
                    f32::max(0.0, 
                        f32::min(1.0, point[i]))) as u8])?;
        }
    }
    Ok(())
}

fn scene_intersect (orig: Vec3f, dir: Vec3f, spheres: &Vec<Sphere>) -> Option<(Vec3f, Vec3f, Material)> {
    let mut dist = f32::MAX;
    let mut hit = Vec3f::zero();
    let mut n = Vec3f::zero();
    let mut material = Material::zero();
    for i in spheres {
        if let Some(dist_i) = i.ray_intersect(orig, dir) {
            if dist_i < dist {
                dist = dist_i;
                hit = orig + dir*dist_i;
                n = (hit - i.center).normalize();
                material = i.material;
            }
        }
    }
    if dist < 10000.0 {
        Some((hit,n,material))
    } else {
        None
    }
}

fn reflect(i: Vec3f, n: Vec3f) -> Vec3f {
    i - n*2.0f32*(i*n)
}

pub fn cast_ray(orig: Vec3f, dir: Vec3f, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Vec3f {
    if let Some((hit, n, material)) = scene_intersect(orig, dir, spheres) {
        let mut diffuse_light_intencity = 0.0;
        let mut specular_light_intensity = 0.0;
        for light in lights {
            let light_dir = (light.pos - hit).normalize();
            diffuse_light_intencity += light.intensity * f32::max(0.0, light_dir*n);
            let rf = -reflect(-light_dir, n)*dir;
            specular_light_intensity += rf.max(0.0).powf(material.specular_exp)*light.intensity;
        }
        material.diffuse_color * diffuse_light_intencity *
        material.albedo[0] + Vec3f::one()*specular_light_intensity * material.albedo[1]
    } else {
        Vec3f::from([0.2, 0.7, 0.8])
    }
}

pub struct Light{
    pos: Vec3f,
    intensity: f32
}

impl Light {
    pub fn new(pos: Vec3f, intensity: f32) -> Self {
        Self {pos, intensity}
    }
}