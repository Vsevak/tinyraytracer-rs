use crate::{geometry::Vec3f, noise::{fractal_brownian_motion, lerp}};

const SPHERE_RADIUS: f32 = 1.5;
const NOISE_AMP: f32 = 1.0;
const STEPS: usize = 128;

fn signed_dist(p: Vec3f) -> f32 {
    let displacement = -fractal_brownian_motion(p*3.4) * NOISE_AMP;
    p.norm() - (SPHERE_RADIUS + displacement)
}

fn sphere_trace(orig: Vec3f, dir: Vec3f) -> Option<Vec3f> {
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

pub fn ray_march(dir: Vec3f) -> Option<Vec3f> {
    if let Some(p) = sphere_trace(Vec3f::new(0.0,0.0,3.0), dir) {
        let noise_lvl = (SPHERE_RADIUS-p.norm())/NOISE_AMP;
        let light_dir = (Vec3f::new(10.0,10.0,10.0) - p).normalize();
        let light_intensity = f32::max(0.4, light_dir*distance_field_normal(p));
        Some(palette((-0.2+noise_lvl)*2.0)*light_intensity)
    } else {
        None
    }
}

pub fn distance_field_normal(pos: Vec3f) -> Vec3f {
    let eps = 0.1;
    let d = signed_dist(pos);
    let nx = signed_dist(pos + Vec3f::new(eps, 0.0, 0.0)) - d;
    let ny = signed_dist(pos + Vec3f::new(0.0, eps, 0.0)) - d;
    let nz = signed_dist(pos + Vec3f::new(0.0, 0.0, eps)) - d;
    Vec3f::new(nx,ny,nz).normalize()
}

pub fn palette(d: f32) -> Vec3f {
    let yellow = Vec3f::new(1.7, 1.3, 1.0);
    let orange = Vec3f::new(1.0, 0.6, 0.0);
    let red = Vec3f::new(1.0, 0.0, 0.0);
    let darkgray = Vec3f::new(0.2, 0.2, 0.2);
    let gray = Vec3f::new(0.4, 0.4, 0.4);
    let x = 0.0f32.max(1.0f32.min(d));
    match (x*100.0) as i32 {
        0..=25 => lerp(gray, darkgray, x*4.0),
        26..=50 => lerp(darkgray, red, x*4.0 - 1.0),
        51..=75 => lerp(red, orange, x*4.0 - 2.0),
        _ => lerp(orange, yellow, x*4.0 - 3.0)
    }
}