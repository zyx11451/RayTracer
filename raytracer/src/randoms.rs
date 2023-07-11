use crate::{
    hittable::HittableList,
    hittable::{MovingSphere, Sphere},
    material::{Dielectric, Lambertian, Metal},
    vec3::{mul_vec_dot, Color, Point3, Vec3},
};
use rand::Rng;
use std::sync::Arc;
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
pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Arc::new(Lambertian {
        albedo: Color { e: (0.5, 0.5, 0.5) },
    });
    world.add(Arc::new(Sphere {
        center: Point3 {
            e: (0.0, -1000.0, 0.0),
        },
        radius: 1000.0,
        mat_ptr: ground_material,
    }));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center_: Point3 = Point3 {
                e: (
                    (a as f64) + 0.9 * random_double(0.0, 1.0),
                    0.2,
                    (b as f64) + 0.9 * random_double(0.0, 1.0),
                ),
            };
            if (center_ - Point3 { e: (4.0, 0.2, 0.0) }).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo_ = random_vec(0.0, 1.0) * random_vec(0.0, 1.0);
                    let center2 = center_
                        + Vec3 {
                            e: (0.0, random_double(0.0, 0.5), 0.0),
                        };
                    let sphere_material = Arc::new(Lambertian { albedo: albedo_ });
                    world.add(Arc::new(MovingSphere {
                        center0: center_,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    let albedo_ = random_vec(0.5, 1.0);
                    let fuzz_ = random_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal {
                        albedo: albedo_,
                        fuzz: fuzz_,
                    });
                    world.add(Arc::new(Sphere {
                        center: center_,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else {
                    let sphere_material = Arc::new(Dielectric { ir: 1.5 });
                    world.add(Arc::new(Sphere {
                        center: center_,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric { ir: 1.5 });
    world.add(Arc::new(Sphere {
        center: Point3 { e: (0.0, 1.0, 0.0) },
        radius: 1.0,
        mat_ptr: material1,
    }));
    let material2 = Arc::new(Lambertian {
        albedo: Color { e: (0.4, 0.2, 0.1) },
    });
    world.add(Arc::new(Sphere {
        center: Point3 {
            e: (-4.0, 1.0, 0.0),
        },
        radius: 1.0,
        mat_ptr: material2,
    }));
    let material3 = Arc::new(Metal {
        albedo: Color { e: (0.7, 0.6, 0.5) },
        fuzz: 0.0,
    });
    world.add(Arc::new(Sphere {
        center: Point3 { e: (4.0, 1.0, 0.0) },
        radius: 1.0,
        mat_ptr: material3,
    }));
    world
}
