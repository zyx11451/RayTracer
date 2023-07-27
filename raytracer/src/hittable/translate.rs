use crate::aabb::AABB;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Hittable;
#[derive(Clone)]
pub struct Translate<H: Hittable> {
    pub offset: Vec3,
    pub ptr: H,
}
impl<H: Hittable> Hittable for Translate<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray {
            orig: r.orig - self.offset,
            dir: r.dir,
            time: r.time,
        };
        let k = self.ptr.hit(&moved_r, t_min, t_max);
        k.as_ref()?;
        let mut rec = k.unwrap();
        rec.p += self.offset;
        rec.set_face_normal(&moved_r, rec.normal);
        Some(rec)
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
