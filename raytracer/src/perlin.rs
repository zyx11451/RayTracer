use crate::{
    randoms::{random_int, random_vec},
    vec3::{mul_vec_dot, Point3, Vec3},
};

pub struct Perlin {
    pub ranvec: [Vec3; 256],
    pub perm_x: [i32; 256],
    pub perm_y: [i32; 256],
    pub perm_z: [i32; 256],
}
pub fn permute(p: &mut [i32; 256], n: i32) {
    for i in (1..n).rev() {
        let target = random_int(0, i);
        p.swap(i as usize, target as usize)
    }
}
pub fn perlin_generate_perm(p: &mut [i32; 256]) {
    for i in 0..256 {
        p[i as usize] = i;
    }
    permute(p, 256);
}
pub fn trilinear_interp(c: &[f64; 8], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for t in 0..8 {
        let i = (t / 4) as f64;
        let j = ((t % 4) / 2) as f64;
        let k = (t % 2) as f64;
        accum += (i * u + (1.0 - i) * (1.0 - u))
            * (j * v + (1.0 - j) * (1.0 - v))
            * (k * w + (1.0 - k) * (1.0 - w))
            * c[t as usize];
    }
    accum
}
pub fn perlin_interp(c: &[Vec3; 8], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for t in 0..8 {
        let i = (t / 4) as f64;
        let j = ((t % 4) / 2) as f64;
        let k = (t % 2) as f64;
        let weight_v = Vec3 {
            e: (u - i, v - j, w - k),
        };
        accum += (i * uu + (1.0 - i) * (1.0 - uu))
            * (j * vv + (1.0 - j) * (1.0 - vv))
            * (k * ww + (1.0 - k) * (1.0 - ww))
            * mul_vec_dot(c[t as usize], weight_v);
    }
    accum
}
impl Perlin {
    pub fn new() -> Self {
        let mut ans = Perlin {
            ranvec: [Vec3::new(); 256],
            perm_x: [0; 256],
            perm_y: [0; 256],
            perm_z: [0; 256],
        };
        for i in 0..256 {
            ans.ranvec[i] = random_vec(-1.0, 1.0).unit_vector();
        }
        perlin_generate_perm(&mut ans.perm_x);
        perlin_generate_perm(&mut ans.perm_y);
        perlin_generate_perm(&mut ans.perm_z);
        ans
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.e.0 - (p.e.0.floor() as f64);
        let v = p.e.1 - (p.e.1.floor() as f64);
        let w = p.e.2 - (p.e.2.floor() as f64);
        let i = (p.e.0.floor()) as i32;
        let j = (p.e.1.floor()) as i32;
        let k = (p.e.2.floor()) as i32;
        let mut c = [Vec3::new(); 8];
        for t in 0..8 {
            let di = t / 4;
            let dj = (t % 4) / 2;
            let dk = t % 2;
            c[t as usize] = self.ranvec[(self.perm_x[((i + di) & 255) as usize]
                ^ self.perm_y[((j + dj) & 255) as usize]
                ^ self.perm_z[((k + dk) & 255) as usize])
                as usize]
        }
        perlin_interp(&c, u, v, w)
    }
}
impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
