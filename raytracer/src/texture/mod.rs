pub mod checkertexture;
pub mod imagetexture;
pub mod noisetexture;
pub mod objloadingtexture;
pub mod solodcolor;
use crate::vec3::{Color, Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
