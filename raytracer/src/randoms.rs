//随机相关
use crate::vec3::{mul_vec_dot, Vec3};
use rand::Rng;
use std::f64::consts::PI;
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
pub fn min(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}
pub fn max(x: f64, y: f64) -> f64 {
    if x > y {
        x
    } else {
        y
    }
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
pub fn random_in_semi_sphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if mul_vec_dot(in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
pub fn random_in_unit_disk() -> Vec3 {
    let mut p = Vec3 {
        e: (random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0),
    };
    loop {
        if p.length_square() < 1.0 {
            break;
        }
        p = Vec3 {
            e: (random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0),
        };
    }
    p
}
pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double(0.0, 1.0);
    let r2 = random_double(0.0, 1.0);
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3 { e: (x, y, z) }
}
pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
    let r1 = random_double(0.0, 1.0);
    let r2 = random_double(0.0, 1.0);
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();
    Vec3 { e: (x, y, z) }
}
pub fn random_int(min: i32, max: i32) -> i32 {
    (random_double(min as f64, (max + 1) as f64)) as i32
}
