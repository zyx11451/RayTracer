use std::f64::consts::PI;

use crate::{
    randoms::random_cosine_direction,
    vec3::{mul_vec_dot, Onb, Vec3},
};

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
pub struct CosinePdf {
    pub uvw: Onb,
}
impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = mul_vec_dot(direction.unit_vector(), self.uvw.axis_z);
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&random_cosine_direction())
    }
}
