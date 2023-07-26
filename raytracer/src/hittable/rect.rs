use crate::aabb::AABB;
use crate::material::material::Material;
use crate::randoms::random_double;
use crate::ray::Ray;
use crate::vec3::mul_vec_dot;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Hittable;
use std::f64::INFINITY;
#[derive(Clone)]
pub struct XyRect<M: Clone + Material> {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mp: M,
}
impl<M: Clone + Material + 'static> Hittable for XyRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.e.2) / r.dir.e.2;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.e.0 + t * r.dir.e.0;
        let y = r.orig.e.1 + t * r.dir.e.1;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord::new();
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3 { e: (0.0, 0.0, 1.0) };
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = &self.mp;
        rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            minimum: Point3 {
                e: (self.x0, self.y0, self.k - 0.00001),
            },
            maximum: Point3 {
                e: (self.x1, self.y1, self.k + 0.00001),
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
        let rec = k.unwrap();
        let area = (self.x1 - self.x0) * (self.y1 - self.y0);
        let distance_squared = rec.t * rec.t * v.length_square();
        let cosine = (mul_vec_dot(*v, rec.normal) / v.length()).abs();
        distance_squared / (cosine * area)
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Point3 {
            e: (
                random_double(self.x0, self.x1),
                self.k,
                random_double(self.y0, self.y1),
            ),
        };
        random_point - *o
    }
}
#[derive(Clone)]
pub struct XzRect<M: Clone + Material> {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: M,
}
impl<M: 'static + Clone + Material> Hittable for XzRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.e.1) / r.dir.e.1;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.e.0 + t * r.dir.e.0;
        let z = r.orig.e.2 + t * r.dir.e.2;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new();
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3 { e: (0.0, 1.0, 0.0) };
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = &self.mp;
        rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            minimum: Point3 {
                e: (self.x0, self.k - 0.00001, self.z0),
            },
            maximum: Point3 {
                e: (self.x1, self.k + 0.00001, self.z1),
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
        let rec = k.unwrap();
        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.length_square();
        let cosine = (mul_vec_dot(*v, rec.normal) / v.length()).abs();
        distance_squared / (cosine * area)
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Point3 {
            e: (
                random_double(self.x0, self.x1),
                self.k,
                random_double(self.z0, self.z1),
            ),
        };
        random_point - *o
    }
}
#[derive(Clone)]
pub struct YzRect<M: Clone + Material> {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: M,
}
impl<M: 'static + Clone + Material> Hittable for YzRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.e.0) / r.dir.e.0;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig.e.1 + t * r.dir.e.1;
        let z = r.orig.e.2 + t * r.dir.e.2;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord::new();
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3 { e: (1.0, 0.0, 0.0) };
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = &self.mp;
        rec.p = r.at(t);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            minimum: Point3 {
                e: (self.k - 0.00001, self.y0, self.z0),
            },
            maximum: Point3 {
                e: (self.k + 0.00001, self.y1, self.z1),
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
        let rec = k.unwrap();
        let area = (self.y1 - self.y0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.length_square();
        let cosine = (mul_vec_dot(*v, rec.normal) / v.length()).abs();
        distance_squared / (cosine * area)
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Point3 {
            e: (
                random_double(self.y0, self.y1),
                self.k,
                random_double(self.z0, self.z1),
            ),
        };
        random_point - *o
    }
}
