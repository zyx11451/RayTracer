use crate::texture::texture::Texture;
use crate::vec3::{Color, Point3};
#[derive(Clone)]
pub struct CheckerTexture<T1: Texture, T2: Texture> {
    pub odd: T1,
    pub even: T2,
}
impl<T1: Texture, T2: Texture> Texture for CheckerTexture<T1, T2> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.e.0).sin() * (10.0 * p.e.1).sin() * (10.0 * p.e.2).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
