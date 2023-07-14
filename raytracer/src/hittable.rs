use crate::aabb::surrounding_box;
use crate::aabb::AABB;
use crate::material::Dielectric;
use crate::material::Isotropic;
use crate::material::Material;
use crate::randoms::random_double;
use crate::texture::Texture;
use crate::vec3::mul_num;

use super::ray::Ray;
use super::vec3::mul_vec_dot;
use super::vec3::Point3;
use super::vec3::Vec3;
use std::f64::consts::PI;
use std::f64::INFINITY;
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
    pub u: f64,
    pub v: f64,
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
            mat_ptr: (Arc::new(Dielectric { ir: 0.0 })),
            u: 0.0,
            v: 0.0,
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
pub struct Sphere<M:Clone+Material>{
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: M,
}
impl<M:Clone+Material> Sphere<M> {
    pub fn get_sphere_uv(&self, p: &Point3, u: &mut f64, v: &mut f64) {
        let theda = (-p.e.1).acos();
        let phi = (-p.e.2).atan2(p.e.0) + PI;
        *u = phi / (2.0 * PI);
        *v = theda / PI;
    }
}
impl<M:'static+Clone+Material> Hittable for Sphere<M> {
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
        self.get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat_ptr = Arc::new(self.mat_ptr.clone());
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
pub struct MovingSphere<M:Clone+Material> {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: M,
}
impl<M:Clone+Material> MovingSphere<M> {
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + mul_num(
                self.center1 - self.center0,
                (time - self.time0) / (self.time1 - self.time0),
            )
    }
}
impl<M:'static+Clone+Material> Hittable for MovingSphere<M> {
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
        rec.mat_ptr = Arc::new(self.mat_ptr.clone());
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
pub struct XyRect<M:Clone+Material> {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mp: M,
}
impl<M:Clone+Material+'static> Hittable for XyRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.e.2) / r.dir.e.2;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.e.0 + t * r.dir.e.0;
        let y = r.orig.e.1 + t * r.dir.e.1;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3 { e: (0.0, 0.0, 1.0) };
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = Arc::new(self.mp.clone());
        rec.p = r.at(t);
        true
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
}
pub struct XzRect<M:Clone+Material> {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: M,
}
impl<M:'static+Clone+Material> Hittable for XzRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.e.1) / r.dir.e.1;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.e.0 + t * r.dir.e.0;
        let z = r.orig.e.2 + t * r.dir.e.2;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3 { e: (0.0, 1.0, 0.0) };
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = Arc::new(self.mp.clone());
        rec.p = r.at(t);
        true
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
}
pub struct YzRect<M:Clone+Material> {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mp: M,
}
impl<M:'static+Clone+Material> Hittable for YzRect<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - r.orig.e.0) / r.dir.e.0;
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.orig.e.1 + t * r.dir.e.1;
        let z = r.orig.e.2 + t * r.dir.e.2;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal = Vec3 { e: (1.0, 0.0, 0.0) };
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = Arc::new(self.mp.clone());
        rec.p = r.at(t);
        true
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
}
pub struct MyBox<M:Clone+Material> {
    pub box_min: Point3,
    pub box_max: Point3,
    pub mat_ptr:M,
    pub sides: HittableList,
}
impl<M:'static+Clone+Material> MyBox<M> {
    pub fn new(p0: &Point3, p1: &Point3, ptr: M) -> Self {
        let box_min_ = p0;
        let box_max_ = p1;
        let mut ans = MyBox {
            box_min: *box_min_,
            box_max: *box_max_,
            mat_ptr:ptr.clone(),
            sides: HittableList::new(),
        };
        ans.sides.add(Arc::new(XyRect {
            x0: p0.e.0,
            x1: p1.e.0,
            y0: p0.e.1,
            y1: p1.e.1,
            k: p1.e.2,
            mp: ptr.clone(),
        }));
        ans.sides.add(Arc::new(XyRect {
            x0: p0.e.0,
            x1: p1.e.0,
            y0: p0.e.1,
            y1: p1.e.1,
            k: p0.e.2,
            mp: ptr.clone(),
        }));
        ans.sides.add(Arc::new(XzRect {
            x0: p0.e.0,
            x1: p1.e.0,
            z0: p0.e.2,
            z1: p1.e.2,
            k: p1.e.1,
            mp: ptr.clone(),
        }));
        ans.sides.add(Arc::new(XzRect {
            x0: p0.e.0,
            x1: p1.e.0,
            z0: p0.e.2,
            z1: p1.e.2,
            k: p0.e.1,
            mp: ptr.clone(),
        }));
        ans.sides.add(Arc::new(YzRect {
            y0: p0.e.1,
            y1: p1.e.1,
            z0: p0.e.2,
            z1: p1.e.2,
            k: p1.e.0,
            mp: ptr.clone(),
        }));
        ans.sides.add(Arc::new(YzRect {
            y0: p0.e.1,
            y1: p1.e.1,
            z0: p0.e.2,
            z1: p1.e.2,
            k: p0.e.0,
            mp: ptr,
        }));
        ans
    }
}
impl<M:Clone+Material> Hittable for MyBox<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            minimum: self.box_min,
            maximum: self.box_max,
        };
        true
    }
}
pub struct Translate<H:Hittable> {
    pub offset: Vec3,
    pub ptr: H,
}
impl<H:Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray {
            orig: r.orig - self.offset,
            dir: r.dir,
            time: r.time,
        };
        if !(self.ptr.hit(&moved_r, t_min, t_max, rec)) {
            return false;
        }
        rec.p += self.offset;
        rec.set_face_normal(&moved_r, rec.normal);
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !(self.ptr.bounding_box(time0, time1, output_box)) {
            return false;
        }
        *output_box = AABB {
            minimum: output_box.minimum + self.offset,
            maximum: output_box.maximum + self.offset,
        };
        true
    }
}
pub struct RotateY<H:Hittable> {
    pub ptr: H,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AABB,
}
impl<H:Hittable> RotateY<H> {
    pub fn new(p:  H, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta_ = radians.sin();
        let cos_theta_ = radians.cos();
        let mut bbox_ = AABB {
            minimum: Vec3::new(),
            maximum: Vec3::new(),
        };
        let hasbox_ = p.bounding_box(0.0, 1.0, &mut bbox_);
        let mut min = Point3 {
            e: (INFINITY, INFINITY, INFINITY),
        };
        let mut max = Point3 {
            e: (-INFINITY, -INFINITY, -INFINITY),
        };
        for t in 0..8 {
            let i = t / 4;
            let j = (t % 4) / 2;
            let k = t % 2;
            let x = (i as f64) * bbox_.maximum.e.0 + ((1 - i) as f64) * bbox_.minimum.e.0;
            let y = (j as f64) * bbox_.maximum.e.1 + ((1 - j) as f64) * bbox_.minimum.e.1;
            let z = (k as f64) * bbox_.maximum.e.2 + ((1 - k) as f64) * bbox_.minimum.e.2;
            let new_x = cos_theta_ * x + sin_theta_ * z;
            let new_z = -sin_theta_ * x + cos_theta_ * z;
            let tester = Vec3 {
                e: (new_x, y, new_z),
            };
            min.e.0 = crate::randoms::min(min.e.0, tester.e.0);
            max.e.0 = crate::randoms::max(max.e.0, tester.e.0);
            min.e.1 = crate::randoms::min(min.e.1, tester.e.1);
            max.e.1 = crate::randoms::max(max.e.1, tester.e.1);
            min.e.2 = crate::randoms::min(min.e.2, tester.e.2);
            max.e.2 = crate::randoms::max(max.e.0, tester.e.2);
        }
        Self {
            ptr: p,
            sin_theta: sin_theta_,
            cos_theta: cos_theta_,
            hasbox: hasbox_,
            bbox: AABB {
                minimum: min,
                maximum: max,
            },
        }
    }
}
impl<H:Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.orig;
        let mut direction = r.dir;
        origin.e.0 = self.cos_theta * r.orig.e.0 - self.sin_theta * r.orig.e.2;
        origin.e.2 = self.sin_theta * r.orig.e.0 + self.cos_theta * r.orig.e.2;
        direction.e.0 = self.cos_theta * r.dir.e.0 - self.sin_theta * r.dir.e.2;
        direction.e.2 = self.sin_theta * r.dir.e.0 + self.cos_theta * r.dir.e.2;
        let rotated_r = Ray {
            orig: origin,
            dir: direction,
            time: r.time,
        };
        if !(self.ptr.hit(&rotated_r, t_min, t_max, rec)) {
            return false;
        }
        let mut p = rec.p;
        let mut normal = rec.normal;
        p.e.0 = self.cos_theta * rec.p.e.0 + self.sin_theta * rec.p.e.2;
        p.e.2 = -self.sin_theta * rec.p.e.0 + self.cos_theta * rec.p.e.2;
        normal.e.0 = self.cos_theta * rec.normal.e.0 + self.sin_theta * rec.normal.e.2;
        normal.e.2 = -self.sin_theta * rec.normal.e.0 + self.cos_theta * rec.normal.e.2;
        rec.p = p;
        rec.set_face_normal(&rotated_r, normal);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
}
pub struct ConstantMedium<H:Hittable,M:Material> {
    pub boundary: H,
    pub phase_function: M,
    pub neg_inv_density: f64,
}
impl<H:Hittable,T:Clone+Texture> ConstantMedium<H,Isotropic<T>> {
    pub fn new(b: H, d: f64, a: T) -> Self {
        Self {
            boundary: b,
            phase_function: Isotropic { albedo: a},
            neg_inv_density: (-1.0 / d),
        }
    }
}
impl<H:Hittable,T:'static+Clone+Texture> Hittable for ConstantMedium<H,Isotropic<T>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();
        if !(self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1)) {
            return false;
        }
        if !(self.boundary.hit(r, rec1.t + 0.0001, INFINITY, &mut rec2)) {
            return false;
        }
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double(0.0, 1.0).log(2.0);
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3 { e: (1.0, 0.0, 0.0) };
        rec.front_face = true;
        rec.mat_ptr = Arc::new(self.phase_function.clone());
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}