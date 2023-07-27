use crate::material::Material;
use crate::material::ScatterRecord;
use crate::randoms::{min, random_double};
use crate::vec3::Color;
use crate::vec3::{mul_vec_dot, reflect, refract};
use crate::{hittable::HitRecord, ray::Ray};
#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64,
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 * r0 + (1.0 - r0 * r0) * ((1.0 - cosine).powf(5.0))
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflection_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.dir.unit_vector();
        let cos_theta = min(mul_vec_dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = reflection_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || reflectance(cos_theta, reflection_ratio) > random_double(0.0, 1.0)
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, reflection_ratio)
        };
        Some(ScatterRecord {
            specular_ray: Ray {
                orig: rec.p,
                dir: direction,
                time: r_in.time,
            },
            is_specular: true,
            attenuation: Color { e: (1.0, 1.0, 1.0) },
            pdf_ptr: None,
        })
    }
}
