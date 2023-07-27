use crate::texture::Texture;
use crate::vec3::{Color, Point3};

#[derive(Clone)]
pub struct SolidColor {
    pub color_value: Color,
}
impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color_value: color }
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}
