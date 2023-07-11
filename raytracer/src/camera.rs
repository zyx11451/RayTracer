use crate::{
    randoms::{random_double, random_in_unit_disk},
    ray::Ray,
    vec3::{mul_num, mul_vec_cross, Point3, Vec3},
};
#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizonal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    //w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
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
            time0: 0.0,
            time1: 1.0,
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
        _time0: f64,
        _time1: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w_ = (lookfrom - lookat).unit_vector();
        let u_ = mul_vec_cross(vup, w_).unit_vector();
        let v_ = mul_vec_cross(w_, u_);
        let origin_ = lookfrom;
        let horizonal_ = u_ * viewport_width * focus_dist;
        let vertical_ = v_ * viewport_height * focus_dist;
        let lower_left_corner_ = origin_ - horizonal_ / 2.0 - vertical_ / 2.0 - w_ * focus_dist;
        Self {
            origin: origin_,
            lower_left_corner: lower_left_corner_,
            horizonal: horizonal_,
            vertical: vertical_,
            //w: w,
            u: u_,
            v: v_,
            lens_radius: aperture / 2.0,
            time0: _time0,
            time1: _time1,
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
            time: random_double(self.time0, self.time1),
        }
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
