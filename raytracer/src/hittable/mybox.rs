use super::rect::XyRect;
use super::rect::XzRect;
use super::rect::YzRect;
use crate::aabb::AABB;
use crate::material::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;
use crate::HitRecord;
use crate::Hittable;
use crate::HittableList;
pub struct MyBox<M: Clone + Material> {
    pub box_min: Point3,
    pub box_max: Point3,
    pub mat_ptr: M,
    pub sides: HittableList,
}
impl<M: 'static + Clone + Material> MyBox<M> {
    pub fn new(p0: &Point3, p1: &Point3, ptr: M) -> Self {
        let box_min_ = p0;
        let box_max_ = p1;
        let mut ans = MyBox {
            box_min: *box_min_,
            box_max: *box_max_,
            mat_ptr: ptr.clone(),
            sides: HittableList::new(),
        };
        ans.sides.add(Box::new(XyRect {
            x0: p0.e.0,
            x1: p1.e.0,
            y0: p0.e.1,
            y1: p1.e.1,
            k: p1.e.2,
            mp: ptr.clone(),
        }));
        ans.sides.add(Box::new(XyRect {
            x0: p0.e.0,
            x1: p1.e.0,
            y0: p0.e.1,
            y1: p1.e.1,
            k: p0.e.2,
            mp: ptr.clone(),
        }));
        ans.sides.add(Box::new(XzRect {
            x0: p0.e.0,
            x1: p1.e.0,
            z0: p0.e.2,
            z1: p1.e.2,
            k: p1.e.1,
            mp: ptr.clone(),
        }));
        ans.sides.add(Box::new(XzRect {
            x0: p0.e.0,
            x1: p1.e.0,
            z0: p0.e.2,
            z1: p1.e.2,
            k: p0.e.1,
            mp: ptr.clone(),
        }));
        ans.sides.add(Box::new(YzRect {
            y0: p0.e.1,
            y1: p1.e.1,
            z0: p0.e.2,
            z1: p1.e.2,
            k: p1.e.0,
            mp: ptr.clone(),
        }));
        ans.sides.add(Box::new(YzRect {
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
impl<M: Clone + Material> Hittable for MyBox<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB {
            minimum: self.box_min,
            maximum: self.box_max,
        };
        true
    }
}
