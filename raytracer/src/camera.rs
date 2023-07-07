use crate::{
    ray::Ray,
    vec3::{mul_num, mul_vec_cross, Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizonal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin_ = Point3 { e: (0.0, 0.0, 0.0) };
        let horizonal_ = Vec3 {
            e: (viewport_width, 0.0, 0.0),
        };
        let vertical_ = Vec3 {
            e: (0.0, viewport_height, 0.0),
        };
        let lower_left_corner_ = origin_
            - horizonal_ / 2.0
            - vertical_ / 2.0
            - Vec3 {
                e: (0.0, 0.0, focal_length),
            };
        Self {
            origin: origin_,
            lower_left_corner: lower_left_corner_,
            horizonal: horizonal_,
            vertical: vertical_,
        }
    }
    pub fn new_cam(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = mul_vec_cross(vup, w).unit_vector();
        let v = mul_vec_cross(w, u);
        let origin_ = lookfrom;
        let horizonal_ = u * viewport_width;
        let vertical_ = v * viewport_height;
        let lower_left_corner_ = origin_ - horizonal_ / 2.0 - vertical_ / 2.0 - w;
        Self {
            origin: origin_,
            lower_left_corner: lower_left_corner_,
            horizonal: horizonal_,
            vertical: vertical_,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            orig: (self.origin),
            dir: (self.lower_left_corner + mul_num(self.horizonal, s) + mul_num(self.vertical, t)
                - self.origin),
        }
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
