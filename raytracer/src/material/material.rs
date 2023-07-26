use crate::pdf:: Pdf;
use crate::vec3::{Color, Point3};
use crate::{hittable::hittable::HitRecord, ray::Ray};

pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf_ptr: Option<Box<dyn Pdf>>,
}
pub trait Material {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        0.0
    }
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color { e: (0.0, 0.0, 0.0) }
    }
}