use crate::material::Material;
use crate::material::ScatterRecord;
use crate::texture::{solodcolor::SolidColor, Texture};
use crate::vec3::{Color, Point3};
use crate::{hittable::HitRecord, ray::Ray};
#[derive(Clone)]
pub struct DiffuseLight<T: Texture> {
    pub emit: T,
}
impl DiffuseLight<SolidColor> {
    pub fn new(c: Color) -> Self {
        Self {
            emit: SolidColor { color_value: c },
        }
    }
}
impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color { e: (0.0, 0.0, 0.0) }
        }
    }
}
