use crate::{geometry::Vec3f, noise::fractal_brownian_motion};

const SPHERE_RADIUS: f32 = 1.5;
const NOISE_AMP: f32 = 1.0;
const STEPS: usize = 128;

fn signed_dist(p: Vec3f) -> f32 {
    let displacement = -fractal_brownian_motion(p*3.4) * NOISE_AMP;
    p.norm() - (SPHERE_RADIUS + displacement)
}

pub fn sphere_trace(orig: Vec3f, dir: Vec3f) -> Option<Vec3f> {
    if orig*orig - f32::powi(orig*dir, 2) > f32::powi(SPHERE_RADIUS, 2) {
        return None;
    }

    let mut pos = orig;
    for _ in 0..STEPS {
        let d = signed_dist(pos);
        if d < 0.0 {
            return Some(pos);
        }
        pos = pos + dir*f32::max(d*0.1, 0.01);
    }
    None
}

pub fn distance_field_normal(pos: Vec3f) -> Vec3f {
    let eps = 0.1;
    let d = signed_dist(pos);
    let nx = signed_dist(pos + Vec3f::new(eps, 0.0, 0.0)) - d;
    let ny = signed_dist(pos + Vec3f::new(0.0, eps, 0.0)) - d;
    let nz = signed_dist(pos + Vec3f::new(0.0, 0.0, eps)) - d;
    Vec3f::new(nx,ny,nz).normalize()
}