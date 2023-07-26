use crate::aabb::AABB;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::HitRecord;
use crate::Hittable;
pub struct FlipFace<H: Hittable> {
    pub ptr: H,
}
impl<H: Hittable> Hittable for FlipFace<H> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let k = self.ptr.hit(r, t_min, t_max);
        k.as_ref()?;
        let mut rec = k.unwrap();
        rec.front_face = !rec.front_face;
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.ptr.bounding_box(time0, time1, output_box)
    }
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        self.ptr.pdf_value(o, v)
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        self.ptr.random(o)
    }
}
