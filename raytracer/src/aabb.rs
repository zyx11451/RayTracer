use std::mem::swap;

use crate::{
    randoms::{max, min},
    ray::Ray,
    vec3::Point3,
};

#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}
impl AABB {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let inv_d = 1.0 / r.dir.e.0;
        let mut t0 = (self.minimum.e.0 - r.orig.e.0) * inv_d;
        let mut t1 = (self.maximum.e.0 - r.orig.e.0) * inv_d;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        let mut temp_min = t_min;
        let mut temp_max = t_max;
        temp_min = if t0 > temp_min { t0 } else { temp_min };
        temp_max = if t1 < temp_max { t1 } else { temp_max };
        if temp_max <= temp_min {
            return false;
        }
        let inv_d = 1.0 / r.dir.e.1;
        let mut t0 = (self.minimum.e.1 - r.orig.e.1) * inv_d;
        let mut t1 = (self.maximum.e.1 - r.orig.e.1) * inv_d;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        temp_min = if t0 > temp_min { t0 } else { temp_min };
        temp_max = if t1 < temp_max { t1 } else { temp_max };
        if temp_max <= temp_min {
            return false;
        }
        let inv_d = 1.0 / r.dir.e.2;
        let mut t0 = (self.minimum.e.2 - r.orig.e.2) * inv_d;
        let mut t1 = (self.maximum.e.2 - r.orig.e.2) * inv_d;
        if inv_d < 0.0 {
            swap(&mut t0, &mut t1);
        }
        temp_min = if t0 > temp_min { t0 } else { temp_min };
        temp_max = if t1 < temp_max { t1 } else { temp_max };
        if temp_max <= temp_min {
            return false;
        }
        true
    }
}
pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Point3 {
        e: (
            min(box0.minimum.e.0, box1.minimum.e.0),
            min(box0.minimum.e.1, box1.minimum.e.1),
            min(box0.minimum.e.2, box1.minimum.e.2),
        ),
    };
    let big = Point3 {
        e: (
            max(box0.maximum.e.0, box1.maximum.e.0),
            max(box0.maximum.e.1, box1.maximum.e.1),
            max(box0.maximum.e.2, box1.maximum.e.2),
        ),
    };
    AABB {
        minimum: small,
        maximum: big,
    }
}
