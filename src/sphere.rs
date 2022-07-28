use crate::render::Vec3f;

pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32
}

impl Sphere {
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