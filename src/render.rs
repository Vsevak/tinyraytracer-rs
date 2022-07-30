use std::fs::{File};
use std::io::{prelude::*, Error, BufWriter};
use std::mem::swap;
use std::path::Path;

use crate::geometry::Vec3f;
use crate::march::{ray_march};
use crate::sphere::{Sphere, Material};

pub struct Frame {
    framebuffer:Vec<Vec3f>, 
    width: usize,
    height:usize
}

pub struct View {
    width: usize,
    height:usize,
    fov: f32
}

impl View {
    pub fn new(width:usize, height: usize, fov: f32) -> Self {
        Self { width, height, fov }
    }

    pub fn render(self, scene: Scene) -> Frame {
        let fheight = self.height as f32;
        let fwidth = self.width as f32;
        let mut framebuffer: Vec<Vec3f> = Vec::new();
        framebuffer.resize(self.width*self.height, Vec3f::zero());
    
        for j in 0..self.height {
            for i in 0..self.width {
                let fi = i as f32;
                let fj = j as f32;
                let x =  (fi + 0.5) - fwidth / 2.0;
                let y = -(fj + 0.5) + fheight / 2.0;
                let z = -fheight/(2.0*f32::tan(self.fov/2.0));
                let dir = Vec3f::from([x, y, z]).normalize();
                let pixel = scene.cast_ray(Vec3f::zero(), dir,  10);
                let max = pixel[0].max(pixel[1].max(pixel[2]));
                framebuffer[i+j*self.width] =  if max > 1.0 {
                    pixel * (1.0/max)
                } else {
                    pixel
                };
                // framebuffer[i+j*width] = if let Some((x, a)) = ray_march(dir) {
                //     //Vec3f::new(0.2, 0.7, 0.8)
                //     //framebuffer[i+j*width] = framebuffer[i+j*width]*(1.0-a) + x*a;
                //     x
                // } else {
                //     Vec3f::new(0.2, 0.7, 0.8)
                // }
            }
        }
        Frame { framebuffer, width: self.width, height: self.height }
    }
}

impl Frame {
    pub fn save(&self) -> Result<(), Error> {
        let path = Path::new("./out.ppm");
        let mut file = File::create(&path)?;
        write!(file, "P6\n{} {}\n255\n", self.width, self.height)?;
        let mut file_buff = BufWriter::new(file);
        for point in &self.framebuffer {
            for i in 0..3 {
                file_buff.write(
                    &[(255.0f32 * 
                        f32::max(0.0, f32::min(1.0, point[i]))) as u8])?;
                    }
                }
                Ok(())
            }
}

pub struct Scene<'a> {
    spheres: Vec<Sphere<'a>>,
    lights: Vec<Light>
}

impl<'a> Scene<'a> {
    pub fn new(spheres: Vec<Sphere<'a>>, lights: Vec<Light>) -> Self {
        Self { spheres, lights }
    }

    fn scene_intersect (&self, orig: Vec3f, dir: Vec3f) -> Option<(Vec3f, Vec3f, Material)> {
        let mut spheres_dist = f32::MAX;
        let mut hit = Vec3f::zero();
        let mut n = Vec3f::zero();
        let mut material = Material::default();
        for i in &self.spheres {
            if let Some(dist_i) = i.ray_intersect(orig, dir) {
                if dist_i < spheres_dist {
                    spheres_dist = dist_i;
                    hit = orig + dir*dist_i;
                    n = (hit - i.center).normalize();
                    material = i.material.clone();
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

    fn cast_ray(&self, orig: Vec3f, dir: Vec3f, depth: usize) -> Vec3f {
        if depth == 0 {
            return Vec3f::new(0.2, 0.7, 0.8)
        }
        let (hit, n, material) = match self.scene_intersect(orig, dir) {
            Some((x,y,z)) => (x,y,z),
            None => return Vec3f::new(0.2, 0.7, 0.8)
        };
    
        let reflect_dir = reflect(dir, n).normalize();
        let reflect_orig = normal_offset(hit, n, reflect_dir);
        let reflect_color = self.cast_ray(reflect_orig, reflect_dir, depth-1);
    
        let refract_dir = refract(dir, n, material.refractive_index).normalize();
        let refract_orig = normal_offset(hit, n, refract_dir);
        let refract_color = self.cast_ray(refract_orig, refract_dir, depth-1);
    
        let mut diffuse_light_intencity = 0.0;
        let mut specular_light_intensity = 0.0;
        for light in &self.lights {
            let light_dir = (light.pos - hit).normalize();
            let light_dist = (light.pos - hit).norm();
            let shadow_orig = normal_offset(hit, n, light_dir); 
            if let Some((shadow_pt,_,_)) = self.scene_intersect(shadow_orig, light_dir) {
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