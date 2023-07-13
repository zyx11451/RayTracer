use crate::{
    randoms::{random_double, random_int},
    vec3::Point3,
};

pub struct Perlin {
    pub ranfloat: [f64; 256],
    pub perm_x: [i32; 256],
    pub perm_y: [i32; 256],
    pub perm_z: [i32; 256],
}
pub fn permute(p: &mut [i32; 256], n: i32) {
    for i in (1..n).rev() {
        let target = random_int(0, i);
        /*let tmp = p[i as usize];
        p[i as usize] = p[target as usize];
        p[target as usize] = tmp;*/
        p.swap(i as usize, target as usize)
    }
}
pub fn perlin_generate_perm(p: &mut [i32; 256]) {
    for i in 0..256 {
        p[i as usize] = i;
    }
    permute(p, 256);
}
impl Perlin {
    pub fn new() -> Self {
        let mut ans = Perlin {
            ranfloat: [0.0; 256],
            perm_x: [0; 256],
            perm_y: [0; 256],
            perm_z: [0; 256],
        };
        for i in 0..256 {
            ans.ranfloat[i] = random_double(0.0, 1.0);
        }
        perlin_generate_perm(&mut ans.perm_x);
        perlin_generate_perm(&mut ans.perm_y);
        perlin_generate_perm(&mut ans.perm_z);
        ans
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.e.0) as i32) & 255;
        let j = ((4.0 * p.e.1) as i32) & 255;
        let k = ((4.0 * p.e.2) as i32) & 255;
        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }
}
impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
