use std::cmp::Ordering;

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
    randoms::random_int,
    ray::Ray,
    vec3::Vec3,
};
pub struct BvhNode {
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
    pub box0: AABB,
}
impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !(self.box0.hit(r, t_min, t_max)) {
            return None;
        }
        let mut rec = HitRecord::new();
        let hit_left = if self.left.is_some() {
            self.left.as_ref().unwrap().hit(r, t_min, t_max)
        } else {
            None
        };
        let mut hit_any = false;
        if let Some(..) = hit_left {
            hit_any = true;
            rec = hit_left.unwrap();
        }
        let hit_right = if self.right.is_some() {
            self.right
                .as_ref()
                .unwrap()
                .hit(r, t_min, if hit_any { rec.t } else { t_max })
        } else {
            None
        };

        if let Some(..) = hit_right {
            hit_any = true;
            rec = hit_right.unwrap()
        };
        if hit_any {
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.box0;
        true
    }
}
impl BvhNode {
    pub fn new_nodes(
        src_objects: &mut Vec<Box<dyn Hittable>>,
        start: u32,
        end: u32,
        time0: f64,
        time1: f64,
    ) -> Self {
        let left_: Option<Box<dyn Hittable>>;
        let right_: Option<Box<dyn Hittable>>;

        let objects = src_objects;
        let axis = random_int(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;
        if object_span == 1 {
            left_ = Some(objects.remove(start as usize));
            right_ = None;
        } else if object_span == 2 {
            if comparator(&objects[start as usize], &objects[(start + 1) as usize])
                == Ordering::Less
            {
                right_ = Some(objects.remove((start + 1) as usize));
                left_ = Some(objects.remove(start as usize));
            } else {
                left_ = Some(objects.remove((start + 1) as usize));
                right_ = Some(objects.remove(start as usize));
            }
        } else {
            let objects_m = &mut objects[start as usize..end as usize];
            objects_m.sort_by(comparator);

            let mid = start + object_span / 2;
            right_ = Some(Box::new(BvhNode::new_nodes(
                objects, mid, end, time0, time1,
            )));
            left_ = Some(Box::new(BvhNode::new_nodes(
                objects, start, mid, time0, time1,
            )));
        }
        let mut box_left: AABB = AABB {
            minimum: Vec3::new(),
            maximum: Vec3::new(),
        };
        let mut box_right: AABB = AABB {
            minimum: Vec3::new(),
            maximum: Vec3::new(),
        };
        left_
            .as_ref()
            .unwrap()
            .bounding_box(time0, time1, &mut box_left);
        if right_.is_some() {
            right_
                .as_ref()
                .unwrap()
                .bounding_box(time0, time1, &mut box_right)
        } else {
            box_right = box_left;
            false
        };
        let box0_: AABB = surrounding_box(&box_left, &box_right);
        Self {
            left: left_,
            right: right_,
            box0: box0_,
        }
    }
}
#[allow(clippy::borrowed_box)]
pub fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    let mut box_a = AABB {
        minimum: Vec3::new(),
        maximum: Vec3::new(),
    };
    let mut box_b = AABB {
        minimum: Vec3::new(),
        maximum: Vec3::new(),
    };
    a.bounding_box(0.0, 0.0, &mut box_a);
    b.bounding_box(0.0, 0.0, &mut box_b);
    if box_a.minimum.e.0 < box_b.minimum.e.0 {
        Ordering::Less
    } else if box_a.minimum.e.0 > box_b.minimum.e.0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
#[allow(clippy::borrowed_box)]
pub fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    let mut box_a = AABB {
        minimum: Vec3::new(),
        maximum: Vec3::new(),
    };
    let mut box_b = AABB {
        minimum: Vec3::new(),
        maximum: Vec3::new(),
    };
    a.bounding_box(0.0, 0.0, &mut box_a);
    b.bounding_box(0.0, 0.0, &mut box_b);
    if box_a.minimum.e.1 < box_b.minimum.e.1 {
        Ordering::Less
    } else if box_a.minimum.e.1 > box_b.minimum.e.1 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
#[allow(clippy::borrowed_box)]
pub fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    let mut box_a = AABB {
        minimum: Vec3::new(),
        maximum: Vec3::new(),
    };
    let mut box_b = AABB {
        minimum: Vec3::new(),
        maximum: Vec3::new(),
    };
    a.bounding_box(0.0, 0.0, &mut box_a);
    b.bounding_box(0.0, 0.0, &mut box_b);
    if box_a.minimum.e.2 < box_b.minimum.e.2 {
        Ordering::Less
    } else if box_a.minimum.e.2 > box_b.minimum.e.2 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
unsafe impl Send for BvhNode {}
unsafe impl Sync for BvhNode {}
