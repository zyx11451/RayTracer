use crate::material::Material;
use crate::material::ScatterRecord;
use crate::randoms::random_in_unit_sphere;
use crate::vec3::reflect;
use crate::vec3::Color;
use crate::vec3::Vec3;
use crate::{hittable::HitRecord, ray::Ray};
#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected: Vec3 = reflect(r_in.dir.unit_vector(), rec.normal);

        Some(ScatterRecord {
            specular_ray: Ray {
                orig: (rec.p),
                dir: (reflected + random_in_unit_sphere() * self.fuzz),
                time: r_in.time,
            },
            is_specular: true,
            attenuation: self.albedo,
            pdf_ptr: None,
        })
    }
}
