use crate::aabb::AABB;
use crate::material::isotropic::Isotropic;
use crate::material::material::Material;
use crate::random_double;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Hittable;
use std::f64::INFINITY;
#[derive(Clone)]
pub struct ConstantMedium<H: Hittable, M: Material> {
    pub boundary: H,
    pub phase_function: M,
    pub neg_inv_density: f64,
}
impl<H: Hittable, T: Clone + Texture> ConstantMedium<H, Isotropic<T>> {
    pub fn new(b: H, d: f64, a: T) -> Self {
        Self {
            boundary: b,
            phase_function: Isotropic { albedo: a },
            neg_inv_density: (-1.0 / d),
        }
    }
}
impl<H: Hittable, T: 'static + Clone + Texture> Hittable for ConstantMedium<H, Isotropic<T>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let k1 = self.boundary.hit(r, -INFINITY, INFINITY);

        k1.as_ref()?;
        let mut rec1 = k1.unwrap();
        let k2 = self.boundary.hit(r, rec1.t + 0.0001, INFINITY);
        k2.as_ref()?;
        let mut rec2 = k2.unwrap();
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double(0.0, 1.0).log(2.0);
        if hit_distance > distance_inside_boundary {
            return None;
        }
        let mut rec = HitRecord::new();
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3 { e: (1.0, 0.0, 0.0) };
        rec.front_face = true;
        rec.mat_ptr = &self.phase_function;
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
