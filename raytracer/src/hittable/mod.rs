pub mod constantmedium;
pub mod flipface;
pub mod movingsphere;
pub mod mybox;
pub mod rect;
pub mod rotate;
pub mod sphere;
pub mod translate;
pub mod triangle;
use crate::aabb::surrounding_box;
use crate::aabb::AABB;
use crate::material::dielectric::Dielectric;
use crate::material::Material;
use crate::randoms::random_int;

use crate::ray::Ray;
use crate::vec3::mul_vec_dot;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use std::vec::Vec;

static NULL_MATERIAL: Dielectric = Dielectric { ir: 0.0 };

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
    fn pdf_value(&self, _o: &Point3, _v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3 { e: (1.0, 0.0, 0.0) }
    }
}

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: &'a dyn Material,
    pub u: f64,
    pub v: f64,
}
impl<'a> HitRecord<'a> {
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
            mat_ptr: &NULL_MATERIAL,
            u: 0.0,
            v: 0.0,
        }
    }
}
impl<'a> Default for HitRecord<'a> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}
unsafe impl Send for HittableList {}
unsafe impl Sync for HittableList {}
impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            let k = object.hit(r, t_min, closest_so_far);
            if let Some(..) = k {
                temp_rec = k.unwrap();
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }
        if hit_anything {
            Some(temp_rec)
        } else {
            None
        }
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
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(o, v)
        }
        sum
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        self.objects[random_int(0, int_size - 1) as usize].random(o)
    }
}
