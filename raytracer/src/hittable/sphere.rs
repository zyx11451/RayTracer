use crate::aabb::AABB;
use crate::material::Material;
use crate::randoms::random_to_sphere;
use crate::ray::Ray;
use crate::vec3::mul_vec_dot;
use crate::vec3::Onb;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Hittable;
use std::f64::consts::PI;
use std::f64::INFINITY;

#[derive(Clone)]
pub struct Sphere<M: Clone + Material> {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: M,
}
impl<M: 'static + Clone + Material> Sphere<M> {
    pub fn get_sphere_uv(&self, p: &Point3, u: &mut f64, v: &mut f64) {
        let theda = (-p.e.1).acos();
        let phi = (-p.e.2).atan2(p.e.0) + PI;
        *u = phi / (2.0 * PI);
        *v = theda / PI;
    }
}
impl<M: 'static + Clone + Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.orig - self.center;
        let a = r.dir.length_square();
        let half_b = mul_vec_dot(oc, r.dir);
        let c = oc.length_square() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        self.get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = &self.mat_ptr;
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            minimum: self.center
                - Vec3 {
                    e: (self.radius, self.radius, self.radius),
                },
            maximum: self.center
                + Vec3 {
                    e: (self.radius, self.radius, self.radius),
                },
        };
        true
    }
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let k = self.hit(
            &Ray {
                orig: *o,
                dir: *v,
                time: 0.0,
            },
            0.001,
            INFINITY,
        );
        if k.is_none() {
            return 0.0;
        }
        let cos_theta_max =
            (1.0 - self.radius * self.radius / (self.center - *o).length_square()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }
    fn random(&self, o: &Point3) -> Vec3 {
        let direction = self.center - *o;
        let distance_squared = direction.length_square();
        let uvw = Onb::build_from_w(&direction);
        uvw.local_vec(&random_to_sphere(self.radius, distance_squared))
    }
}
