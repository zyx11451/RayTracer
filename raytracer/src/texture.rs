use std::sync::Arc;

use crate::{
    perlin::Perlin,
    vec3::{mul_num, Color, Point3},
};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
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
pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.e.0).sin() * (10.0 * p.e.1).sin() * (10.0 * p.e.2).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
pub struct NoiseTexture {
    pub noise: Perlin,
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        mul_num(Color { e: (1.0, 1.0, 1.0) }, self.noise.noise(p))
    }
}
