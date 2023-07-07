use rand::Rng;

use crate::vec3::Vec3;
pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + rng.gen_range(0.0..(max - min))
}
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
pub fn random_vec(min: f64, max: f64) -> Vec3 {
    Vec3 {
        e: (
            random_double(min, max),
            random_double(min, max),
            random_double(min, max),
        ),
    }
}
pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = random_vec(-1.0, 1.0);
    loop {
        if p.length_square() < 1.0 {
            break;
        }
        p = random_vec(-1.0, 1.0);
    }
    p
}
pub fn random_unit_vec() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}
