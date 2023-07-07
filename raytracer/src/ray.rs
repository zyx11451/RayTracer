use crate::randoms::clamp;

use super::vec3::mul_num;
use super::vec3::Color;
use super::vec3::Point3;
use super::vec3::Vec3;
pub fn write_color(tar: Color, samples_per_pixel: u32) -> image::Rgb<u8> {
    let mut r: f64 = tar.e.0;
    let mut g: f64 = tar.e.1;
    let mut b: f64 = tar.e.2;
    //gamma=2.0
    let scale = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();
    //
    image::Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ])
}
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + mul_num(self.dir, t)
    }
}
