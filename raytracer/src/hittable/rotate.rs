use crate::aabb::AABB;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Hittable;
use std::f64::INFINITY;

#[derive(Clone)]
pub struct RotateY<H: Hittable> {
    pub ptr: H,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub hasbox: bool,
    pub bbox: AABB,
}
impl<H: Hittable> RotateY<H> {
    pub fn new(p: H, angle: f64) -> Self {
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
impl<H: Hittable> Hittable for RotateY<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
        let k = self.ptr.hit(&rotated_r, t_min, t_max);
        k.as_ref()?;
        let mut rec = k.unwrap();
        let mut p = rec.p;
        let mut normal = rec.normal;
        p.e.0 = self.cos_theta * rec.p.e.0 + self.sin_theta * rec.p.e.2;
        p.e.2 = -self.sin_theta * rec.p.e.0 + self.cos_theta * rec.p.e.2;
        normal.e.0 = self.cos_theta * rec.normal.e.0 + self.sin_theta * rec.normal.e.2;
        normal.e.2 = -self.sin_theta * rec.normal.e.0 + self.cos_theta * rec.normal.e.2;
        rec.p = p;
        rec.set_face_normal(&rotated_r, normal);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
}