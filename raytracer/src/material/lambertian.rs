use std::f64::consts::PI;

use crate::material::material::Material;
use crate::material::material::ScatterRecord;
use crate::pdf::CosinePdf;
use crate::texture::texture::Texture;
use crate::vec3::{mul_vec_dot, Onb, Vec3};
use crate::{hittable::hittable::HitRecord, ray::Ray};
#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}
impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            specular_ray: Ray {
                orig: Vec3::new(),
                dir: Vec3::new(),
                time: 0.0,
            },
            is_specular: false,
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf_ptr: Some(Box::new(CosinePdf {
                uvw: Onb::build_from_w(&rec.normal),
            })),
        })
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cosine = mul_vec_dot(rec.normal, scattered.dir.unit_vector());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
