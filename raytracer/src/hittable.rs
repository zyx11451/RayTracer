use crate::aabb::surrounding_box;
use crate::aabb::AABB;
use crate::material::Lambertian;
use crate::material::Material;
use crate::vec3::mul_num;

use super::ray::Ray;
use super::vec3::mul_vec_dot;
use super::vec3::Point3;
use super::vec3::Vec3;
//use std::rc::Rc;
use std::sync::Arc;
use std::vec::Vec;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = mul_vec_dot(r.dir, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
    pub fn new() -> Self {
        Self {
            p: (Vec3::new()),
            normal: (Vec3::new()),
            t: (0.0),
            front_face: (true),
            mat_ptr: (Arc::new(Lambertian {
                albedo: (Vec3 { e: (0.0, 0.0, 0.0) }),
            })),
        }
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}
unsafe impl Send for HittableList {}
unsafe impl Sync for HittableList {}
impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box = AABB {
            minimum: Vec3::new(),
            maximum: Vec3::new(),
        };
        let mut first_box: bool = true;
        for object in &self.objects {
            if !(object.bounding_box(time0, time1, &mut temp_box)) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }
        true
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig - self.center;
        let a = r.dir.length_square();
        let half_b = mul_vec_dot(oc, r.dir);
        let c = oc.length_square() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();
        true
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
}
pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Arc<dyn Material>,
}
impl MovingSphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + mul_num(
                self.center1 - self.center0,
                (time - self.time0) / (self.time1 - self.time0),
            )
    }
}
impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig - self.center(r.time);
        let a = r.dir.length_square();
        let half_b = mul_vec_dot(oc, r.dir);
        let c = oc.length_square() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center(r.time)) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();
        true
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
