use crate::aabb::surrounding_box;
use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::mul_num;
use crate::vec3::mul_vec_dot;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Hittable;
#[derive(Clone)]
pub struct MovingSphere<M: Clone + Material> {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: M,
}
impl<M: Clone + Material> MovingSphere<M> {
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + mul_num(
                self.center1 - self.center0,
                (time - self.time0) / (self.time1 - self.time0),
            )
    }
}
impl<M: 'static + Clone + Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.orig - self.center(r.time);
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
        let outward_normal = (rec.p - self.center(r.time)) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = &self.mat_ptr;
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB {
            minimum: self.center(time0)
                - Vec3 {
                    e: (self.radius, self.radius, self.radius),
                },
            maximum: self.center(time0)
                + Vec3 {
                    e: (self.radius, self.radius, self.radius),
                },
        };
        let box1 = AABB {
            minimum: self.center(time1)
                - Vec3 {
                    e: (self.radius, self.radius, self.radius),
                },
            maximum: self.center(time1)
                + Vec3 {
                    e: (self.radius, self.radius, self.radius),
                },
        };
        *output_box = surrounding_box(&box0, &box1);
        true
    }
}
