use super::vec3::Color;

use super::vec3::mul_num;
use super::vec3::Point3;
use super::vec3::Vec3;
pub fn write_color(tar: Color) -> image::Rgb<u8> {
    let r: f64 = tar.e.0 * 255.999;
    let g: f64 = tar.e.1 * 255.999;
    let b: f64 = tar.e.2 * 255.999;
    image::Rgb([r as u8, g as u8, b as u8])
}
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + mul_num(self.dir, t)
    }
}
