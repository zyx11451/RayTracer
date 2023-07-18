use std::f64::consts::PI;

use crate::{
    hittable::Hittable,
    randoms::{random_cosine_direction, random_double},
    vec3::{mul_vec_dot, Onb, Point3, Vec3},
};

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
#[derive(Clone)]
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
#[derive(Clone)]
pub struct HittablePdf<'a> {
    pub o: Point3,
    pub ptr: &'a dyn Hittable,
}
impl<'a> Pdf for HittablePdf<'a> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct MixturePdf<'a> {
    pub p1: &'a dyn Pdf,
    pub p2: &'a dyn Pdf,
}
impl<'a> Pdf for MixturePdf<'a> {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p1.value(direction) + 0.5 * self.p2.value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_double(0.0, 1.0) < 0.5 {
            self.p1.generate()
        } else {
            self.p2.generate()
        }
    }
}
