use crate::{
    perlin::Perlin,
    vec3::{mul_num, Color, Point3},
};
use crate::texture::texture::Texture;
#[derive(Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        mul_num(
            Color { e: (1.0, 1.0, 1.0) },
            0.5 * (1.0 + (10.0 * self.noise.turb(p, 7) + self.scale * p.e.2).sin()),
        )
    }
}