use crate::material::Material;
use crate::material::ScatterRecord;
use crate::randoms::random_in_unit_sphere;
use crate::texture::{solodcolor::SolidColor, Texture};
use crate::vec3::Color;
use crate::{hittable::HitRecord, ray::Ray};

#[derive(Clone)]
pub struct Isotropic<T: Texture> {
    pub albedo: T,
}
impl Isotropic<SolidColor> {
    pub fn new(a: Color) -> Self {
        Self {
            albedo: SolidColor { color_value: a },
        }
    }
}
impl<T: Texture> Material for Isotropic<T> {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            specular_ray: Ray {
                orig: rec.p,
                dir: random_in_unit_sphere(),
                time: r_in.time,
            },
            is_specular: true,
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf_ptr: None,
        })
    }
}
