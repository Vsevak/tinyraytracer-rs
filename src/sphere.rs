use crate::geometry::{Vec3f};

pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32, material: Material) -> Self {
        Self{
            center,
            radius,
            material
        }
    }
    pub fn ray_intersect(&self, orig: Vec3f , dir: Vec3f) -> Option<f32> {
        let l = self.center - orig;
        let tca = l*dir;
        let d2 = (l*l) - (tca*tca);
        if d2 > self.radius*self.radius {
            return None;
        }
        let thc = (self.radius*self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            None
        } else {
            Some(t0)
        }
    }
}

#[derive(Clone,Copy)]
pub struct Material {
    pub diffuse_color: Vec3f,
    pub albedo: Vec3f,
    pub specular_exp: f32
}

impl Material {
    pub fn zero() -> Self {
        Self { 
            diffuse_color: Vec3f::zero(),
            albedo: Vec3f::zero(),
            specular_exp: 0.0
        }
    }
}