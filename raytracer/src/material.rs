use crate::randoms::{random_in_unit_sphere, random_unit_vec};
use crate::vec3::{mul_vec_dot, reflect, refract, Color, Vec3};

use crate::{hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
pub struct Lambertian {
    pub albedo: Color,
}
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray {
            orig: (rec.p),
            dir: (scatter_direction),
        };
        *attenuation = self.albedo;
        true
    }
}
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = reflect(r_in.dir.unit_vector(), rec.normal);
        *scattered = Ray {
            orig: (rec.p),
            dir: (reflected + random_in_unit_sphere() * self.fuzz),
        };
        *attenuation = self.albedo;
        mul_vec_dot(scattered.dir, rec.normal) > 0.0
    }
}
pub struct Dielectric {
    pub ir: f64,
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: Ray,
        rec: HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color { e: (1.0, 1.0, 1.0) };
        let reflection_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.dir.unit_vector();
        let refracted = refract(unit_direction, rec.normal, reflection_ratio);
        *scattered = Ray {
            orig: rec.p,
            dir: refracted,
        };
        true
    }
}
