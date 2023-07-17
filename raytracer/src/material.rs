use std::f64::consts::PI;

use crate::randoms::{min, random_cosine_direction, random_double, random_in_unit_sphere};
use crate::texture::{SolidColor, Texture};
use crate::vec3::{mul_vec_dot, reflect, refract, Color, Onb, Point3, Vec3};

use crate::{hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &mut Ray) -> f64 {
        1.0
    }
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color { e: (0.0, 0.0, 0.0) }
    }
}
#[derive(Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}
impl<T: Texture> Material for Lambertian<T> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        /*let mut scatter_direction = rec.normal + random_unit_vec();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray {
            orig: (rec.p),
            dir: (scatter_direction),
            time: r_in.time,
        };
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        *pdf=mul_vec_dot(rec.normal, scattered.dir.unit_vector())/PI;
        true*/
        let uvw = Onb::build_from_w(&rec.normal);
        let direction = uvw.local_vec(&random_cosine_direction());
        *scattered = Ray {
            orig: (rec.p),
            dir: (direction),
            time: r_in.time,
        };
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        *pdf = mul_vec_dot(uvw.axis_z, scattered.dir) / PI;
        true
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
#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        let reflected: Vec3 = reflect(r_in.dir.unit_vector(), rec.normal);
        *scattered = Ray {
            orig: (rec.p),
            dir: (reflected + random_in_unit_sphere() * self.fuzz),
            time: r_in.time,
        };
        *attenuation = self.albedo;
        mul_vec_dot(scattered.dir, rec.normal) > 0.0
    }
}
#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64,
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 * r0 + (1.0 - r0 * r0) * ((1.0 - cosine).powf(5.0))
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        *attenuation = Color { e: (1.0, 1.0, 1.0) };
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
        *scattered = Ray {
            orig: rec.p,
            dir: direction,
            time: r_in.time,
        };
        true
    }
}
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
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color { e: (0.0, 0.0, 0.0) }
        }
    }
}
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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        *scattered = Ray {
            orig: rec.p,
            dir: random_in_unit_sphere(),
            time: r_in.time,
        };
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
