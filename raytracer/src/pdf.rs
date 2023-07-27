use std::f64::consts::PI;

use crate::{
    hittable::Hittable,
    randoms::{random_cosine_direction, random_double, random_in_semi_sphere},
    vec3::{mul_vec_dot, Onb, Point3, Vec3},
};

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
#[derive(Clone)]
pub struct TestPdf {
    pub uvw: Onb,
}
impl Pdf for TestPdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = mul_vec_dot(direction.unit_vector(), self.uvw.axis_z);
        if cosine <= 0.0 {
            0.0
        } else {
            1.0 / (2.0 * PI)
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw
            .local_vec(&random_in_semi_sphere(Vec3 { e: (0.0, 0.0, 1.0) }))
    }
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
pub struct HittablePdf<'a, H: Hittable> {
    pub o: Point3,
    pub ptr: &'a H,
}
impl<'a, H: Hittable> Pdf for HittablePdf<'a, H> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}

pub struct MixturePdf<'a, P1: Pdf + ?Sized, P2: Pdf + ?Sized> {
    pub p1: &'a P1,
    pub p2: &'a P2,
}
impl<'a, P1: Pdf + ?Sized, P2: Pdf + ?Sized> Pdf for MixturePdf<'a, P1, P2> {
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
