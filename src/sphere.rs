use crate::geometry::{Vec3f, Vec4f};

pub struct Sphere<'a> {
    pub center: Vec3f,
    pub radius: f32,
    pub material: &'a Material
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3f, radius: f32, material: &'a Material) -> Self {
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
    pub albedo: Vec4f,
    pub refractive_index: f32,
    pub specular_exp: f32
}

impl Default for Material {
    fn default() -> Self {
        Self { 
            diffuse_color: Vec3f::zero(),
            albedo: Vec4f::from([1.0, 0.0, 0.0, 0.0]),
            refractive_index: 1.0,
            specular_exp: 0.0
        }
    }
}