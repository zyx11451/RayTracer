use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
    randoms::random_int,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct BvhNode {
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
    pub box0: AABB,
}
impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !(self.box0.hit(r, t_min, t_max)) {
            return false;
        }
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);
        hit_left || hit_right
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.box0;
        true
    }
}
impl BvhNode {
    pub fn new_nodes(
        src_objects: &Vec<Arc<dyn Hittable>>,
        start: u32,
        end: u32,
        time0: f64,
        time1: f64,
    ) -> Self {
        let left_: Arc<dyn Hittable>;
        let right_: Arc<dyn Hittable>;
        let box0_: AABB;
        let mut objects = src_objects.clone();
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
            left_ = (*objects)[start as usize].clone();
            right_ = (*objects)[start as usize].clone();
        } else if object_span == 2 {
            if comparator(&objects[start as usize], &objects[(start + 1) as usize])
                == Ordering::Less
            {
                left_ = (*objects)[start as usize].clone();
                right_ = (*objects)[(start + 1) as usize].clone();
            } else {
                left_ = (*objects)[(start + 1) as usize].clone();
                right_ = (*objects)[start as usize].clone();
            }
        } else {
            let objects_m = &mut objects[start as usize..end as usize];
            objects_m.sort_by(comparator);
            let mid = start + object_span / 2;
            left_ = Arc::new(BvhNode::new_nodes(&objects, start, mid, time0, time1));
            right_ = Arc::new(BvhNode::new_nodes(&objects, mid, end, time0, time1));
        }
        let mut box_left: AABB = AABB {
            minimum: Vec3::new(),
            maximum: Vec3::new(),
        };
        let mut box_right: AABB = AABB {
            minimum: Vec3::new(),
            maximum: Vec3::new(),
        };
        left_.bounding_box(time0, time1, &mut box_left);
        right_.bounding_box(time0, time1, &mut box_right);
        box0_ = surrounding_box(&box_left, &box_right);
        Self {
            left: left_,
            right: right_,
            box0: box0_,
        }
    }
}
pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
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
pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
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
pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
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
