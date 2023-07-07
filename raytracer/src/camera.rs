use crate::{
    randoms::random_in_unit_disk,
    ray::Ray,
    vec3::{mul_num, mul_vec_cross, Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizonal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    //w: Vec3,
    lens_radius: f64,
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
            u: Vec3::new(),
            v: Vec3::new(),
            //w: Vec3::new(),
            lens_radius: 0.0,
        }
    }
    pub fn new_cam(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = mul_vec_cross(vup, w).unit_vector();
        let v = mul_vec_cross(w, u);
        let origin_ = lookfrom;
        let horizonal_ = u * viewport_width * focus_dist;
        let vertical_ = v * viewport_height * focus_dist;
        let lower_left_corner_ = origin_ - horizonal_ / 2.0 - vertical_ / 2.0 - w * focus_dist;
        Self {
            origin: origin_,
            lower_left_corner: lower_left_corner_,
            horizonal: horizonal_,
            vertical: vertical_,
            //w: w,
            u: u,
            v: v,
            lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.e.0 + self.v * rd.e.1;
        Ray {
            orig: (self.origin + offset),
            dir: (self.lower_left_corner + mul_num(self.horizonal, s) + mul_num(self.vertical, t)
                - self.origin
                - offset),
        }
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
