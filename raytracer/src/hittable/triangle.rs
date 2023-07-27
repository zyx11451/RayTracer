use crate::aabb::AABB;
use crate::material::Material;
use crate::randoms::max;
use crate::randoms::min;
use crate::ray::Ray;
use crate::vec3::mul_vec_cross;
use crate::vec3::mul_vec_dot;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Hittable;
pub struct Triangle<M: Clone + Material> {
    pub a: Point3,
    pub n: Vec3,
    pub pb: Vec3,
    pub pc: Vec3,
    pub mp: M,
    pub bbox: AABB,
    pub uva: (f64, f64),
    pub uvab: (f64, f64),
    pub uvac: (f64, f64),
}
//暂时不能发光
impl<M: Clone + Material> Hittable for Triangle<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oa = self.a - r.orig;
        let t = mul_vec_dot(oa, self.n) / mul_vec_dot(r.dir, self.n);
        if t < t_min || t > t_max {
            return None;
        }
        let p = r.at(t);
        let ap = p - self.a;
        let u = mul_vec_dot(ap, self.pb);
        let v = mul_vec_dot(ap, self.pc);
        if u > 0.0 && v > 0.0 && u + v < 1.0 {
            let rec = HitRecord {
                p,
                normal: self.n,
                t,
                front_face: true,
                mat_ptr: &self.mp,
                u: self.uva.0 + v * self.uvab.0 + u * self.uvac.0,
                v: self.uva.1 + v * self.uvab.1 + u * self.uvac.1,
            };
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        true
    }
}
impl<M: 'static + Clone + Material> Triangle<M> {
    pub fn new(
        a: Point3,
        b: Point3,
        c: Point3,
        ptr: M,
        (ua, va): (f64, f64),
        (ub, vb): (f64, f64),
        (uc, vc): (f64, f64),
    ) -> Self {
        let ab = b - a;
        let ac = c - a;
        let n = mul_vec_cross(ab, ac);
        let l = n.length_square();
        let pb = mul_vec_cross(n, ab) / l;
        let pc = mul_vec_cross(ac, n) / l;
        let mut min_ = Vec3::new();
        let mut max_ = Vec3::new();
        min_.e.0 = min(min(a.e.0, b.e.0), c.e.0) - 0.000001;
        max_.e.0 = max(max(a.e.0, b.e.0), c.e.0) + 0.000001;
        min_.e.1 = min(min(a.e.1, b.e.1), c.e.1) - 0.000001;
        max_.e.1 = max(max(a.e.1, b.e.1), c.e.1) + 0.000001;
        min_.e.2 = min(min(a.e.2, b.e.2), c.e.2) - 0.000001;
        max_.e.2 = max(max(a.e.2, b.e.2), c.e.2) + 0.000001;

        Self {
            a,
            n: n.unit_vector(),
            pb,
            pc,
            mp: ptr,
            bbox: AABB {
                minimum: min_,
                maximum: max_,
            },
            uva: (ua, va),
            uvab: ((ub - ua), (vb - va)),
            uvac: ((uc - ua), (vc - va)),
        }
    }
}
