use std::f32::consts::PI;
use std::fs::{File};
use std::io::{prelude::*, Error, BufWriter};
use std::mem::swap;
use std::path::Path;

use crate::geometry::Vec3f;
use crate::sphere::{Sphere, Material};

pub struct Frame(Vec<Vec3f>, usize, usize);

pub fn render(spheres: Vec<Sphere>, lights: Vec<Light>) -> Frame {
    let width = 1024;
    let height = 768;
    let fov = PI / 3.0;
    let mut framebuffer: Vec<Vec3f> = Vec::new();
    framebuffer.resize(width*height, Vec3f::zero());
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
            let pixel = cast_ray(Vec3f::zero(), dir, &spheres, &lights, 0);
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
                    f32::max(0.0, f32::min(1.0, point[i]))) as u8])?;
        }
    }
    Ok(())
}

fn scene_intersect (orig: Vec3f, dir: Vec3f, spheres: &Vec<Sphere>) -> Option<(Vec3f, Vec3f, Material)> {
    let mut spheres_dist = f32::MAX;
    let mut hit = Vec3f::zero();
    let mut n = Vec3f::zero();
    let mut material = Material::zero();
    for i in spheres {
        if let Some(dist_i) = i.ray_intersect(orig, dir) {
            if dist_i < spheres_dist {
                spheres_dist = dist_i;
                hit = orig + dir*dist_i;
                n = (hit - i.center).normalize();
                material = i.material;
            }
        }
    }

    let mut board_dist = f32::MAX;
    if dir[1].abs() > 1e-3 {
        let d = -(orig[1] + 4.0)/dir[1];
        let pt = orig + dir*d;
        if d>0.0 && pt[0].abs()<10.0 && pt[2]< -10.0 && pt[2]> -30.0 && d<spheres_dist {
            board_dist = d;
            hit = pt;
            n = Vec3f::from([0.0, 1.0, 0.0]);
            material.diffuse_color = if ((0.5*hit[0]+1000.0) as i32 + (0.5*hit[2]) as i32) & 1 == 1 {
                Vec3f::new(0.9, 0.9, 0.9)
            } else {
                Vec3f::new(0.92, 0.69, 0.078)
            }*0.3;
        }
    }

    let dist = f32::min(spheres_dist, board_dist);
    if dist < 10000.0 {
        Some((hit,n,material))
    } else {
        None
    }
}

fn reflect(i: Vec3f, n: Vec3f) -> Vec3f {
    i - n*2.0f32*(i*n)
}

fn refract(i: Vec3f, n: Vec3f, rf_index: f32) -> Vec3f {
    let mut cosi = -f32::max(-1.0, f32::min(1.0, i*n));
    let mut etai = 1.0;
    let mut etat = rf_index;
    let n_i = if cosi < 0.0 {
        cosi = cosi;
        swap(&mut etai, &mut etat);
        -n
    } else {
        n
    };
    let eta  = etai / etat;
    let k = 1.0 - eta*eta*(1.0 - cosi*cosi);
    if k<0.0 {
        Vec3f::zero()
    } else {
        i*eta + n_i*(eta * cosi - k.sqrt())
    }
}

fn cast_ray(orig: Vec3f, dir: Vec3f, spheres: &Vec<Sphere>, lights: &Vec<Light>, depth: usize) -> Vec3f {
    if depth > 4 {
        return Vec3f::new(0.2, 0.7, 0.8)
    }
    let (hit, n, material) = match scene_intersect(orig, dir, spheres) {
        Some((x,y,z)) => (x,y,z),
        None => return Vec3f::new(0.2, 0.7, 0.8)
    };

    let reflect_dir = reflect(dir, n).normalize();
    let reflect_orig = normal_offset(hit, n, reflect_dir);
    let reflect_color = cast_ray(reflect_orig, reflect_dir, spheres, lights, depth+1);

    let refract_dir = refract(dir, n, material.refractive_index).normalize();
    let refract_orig = normal_offset(hit, n, refract_dir);
    let refract_color = cast_ray(refract_orig, refract_dir, spheres, lights, depth+1);

    let mut diffuse_light_intencity = 0.0;
    let mut specular_light_intensity = 0.0;
    for light in lights {
        let light_dir = (light.pos - hit).normalize();
        let light_dist = (light.pos - hit).norm();
        let shadow_orig = normal_offset(hit, n, light_dir); 
        if let Some((shadow_pt,_,_)) = scene_intersect(shadow_orig, light_dir, spheres) {
            if (shadow_pt-shadow_orig).norm() < light_dist {
                continue;
            }
        }
        diffuse_light_intencity += light.intensity * f32::max(0.0, light_dir*n);
        let rf = reflect(light_dir, n)*dir;
        specular_light_intensity += rf.max(0.0).powf(material.specular_exp)*light.intensity;
    }
    material.diffuse_color * diffuse_light_intencity
    * material.albedo[0] + Vec3f::one()*specular_light_intensity * material.albedo[1]
    + reflect_color*material.albedo[2]
    + refract_color*material.albedo[3]
}

// Сдвиг точки в направлении нормали
fn normal_offset(v: Vec3f, n: Vec3f, dir: Vec3f) -> Vec3f {
    v + n*1e-3*(dir*n).signum()
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