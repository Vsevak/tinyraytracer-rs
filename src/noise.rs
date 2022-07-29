use crate::geometry::Vec3f;

#[inline]
fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    v0 + (v1-v0) * 0.0f32.max(1.0f32.min(t))
}

#[inline]
fn hash(n: f32) -> f32 {
    let x = f32::sin(n)*43758.5453;
    x - x.floor()
}

#[inline]
fn noise(x: Vec3f) -> f32 {
    let p = Vec3f::new(x[0].floor(), x[1].floor(), x[2].floor());
    let mut f = x-p;
    f = f*(f*(Vec3f::one()*3.0 - f*2.0));
    let n = p*Vec3f::new(1.0, 57.0, 113.0);
    lerp(lerp(
                lerp(hash(n + 0.0 ), hash(n + 1.0 ), f[0]),
                lerp(hash(n + 57.0), hash(n + 58.0), f[0]), 
                f[1]),
         lerp(
                lerp(hash(n + 113.0), hash(n + 114.0), f[0]),
                lerp(hash(n + 170.0), hash(n + 171.0), f[0]), 
                f[1]), 
          f[2]
        )
}

fn rotate(v: Vec3f) -> Vec3f {
    Vec3f::new(
        Vec3f::new(0.00,  0.80, 0.60)*v, 
        Vec3f::new(-0.80, 0.36, -0.48)*v,
        Vec3f::new(-0.60, -0.48,0.64)*v
    )
}

#[inline]
pub fn fractal_brownian_motion(x: Vec3f) -> f32 {
    let mut p = rotate(x);
    let mut f = 0.0;
    f += 0.5000*noise(p); p = p*2.32;
    f += 0.2500*noise(p); p = p*3.03;
    f += 0.1250*noise(p); p = p*2.61;
    f += 0.0625*noise(p);
    f/0.9375
}
