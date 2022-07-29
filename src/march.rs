use crate::geometry::Vec3f;

const SPHERE_RADIUS: f32 = 1.5;
const STEPS: usize = 128;

fn signed_dist(p: Vec3f) -> f32 {
    p.norm() - SPHERE_RADIUS
}

pub fn sphere_trace(orig: Vec3f, dir: Vec3f) -> Option<Vec3f> {
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
