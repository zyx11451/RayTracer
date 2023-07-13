//随机相关和场景
use crate::{
    hittable::HittableList,
    hittable::{MovingSphere, Sphere, XyRect, XzRect, YzRect},
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    perlin::Perlin,
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor},
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
pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture {
        even: Arc::new(SolidColor::new(Color { e: (0.2, 0.3, 0.1) })),
        odd: Arc::new(SolidColor::new(Color { e: (0.9, 0.9, 0.9) })),
    });
    let ground_material = Arc::new(Lambertian { albedo: checker });
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
                    let sphere_material = Arc::new(Lambertian {
                        albedo: Arc::new(SolidColor {
                            color_value: albedo_,
                        }),
                    });
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
        albedo: Arc::new(SolidColor {
            color_value: Color { e: (0.4, 0.2, 0.1) },
        }),
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
pub fn random_int(min: i32, max: i32) -> i32 {
    (random_double(min as f64, (max + 1) as f64)) as i32
}
pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = Arc::new(CheckerTexture {
        even: Arc::new(SolidColor::new(Color { e: (0.2, 0.3, 0.1) })),
        odd: Arc::new(SolidColor::new(Color { e: (0.9, 0.9, 0.9) })),
    });
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (0.0, -10.0, 0.0),
        },
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: checker.clone(),
        }),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (0.0, 10.0, 0.0),
        },
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian { albedo: checker }),
    }));
    objects
}
pub fn two_perlin_sphere() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture {
        noise: Perlin::new(),
        scale: 4.0,
    });
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (0.0, -1000.0, 0.0),
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 { e: (0.0, 2.0, 0.0) },
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian { albedo: pertext }),
    }));
    objects
}
pub fn earth() -> HittableList {
    let mut objects = HittableList::new();
    let path = std::path::Path::new("raytracer/src/earthmap.jpg");
    let earth_texture = Arc::new(ImageTexture::new(path));

    let earth_surface = Arc::new(Lambertian {
        albedo: earth_texture,
    });
    objects.add(Arc::new(Sphere {
        center: Point3 { e: (0.0, 0.0, 0.0) },
        radius: 2.0,
        mat_ptr: earth_surface,
    }));
    objects
}
pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture {
        noise: Perlin::new(),
        scale: 4.0,
    });
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (0.0, -1000.0, 0.0),
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 { e: (0.0, 2.0, 0.0) },
        radius: 2.0,
        mat_ptr: Arc::new(Lambertian { albedo: pertext }),
    }));
    let difflight = Arc::new(DiffuseLight::new(Color { e: (4.0, 4.0, 4.0) }));
    objects.add(Arc::new(XyRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        mp: difflight.clone(),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 { e: (0.0, 7.0, 0.0) },
        radius: 2.0,
        mat_ptr: difflight,
    }));
    objects
}
pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian {
        albedo: Arc::new(SolidColor {
            color_value: Color {
                e: (0.65, 0.05, 0.05),
            },
        }),
    });
    let white = Arc::new(Lambertian {
        albedo: Arc::new(SolidColor {
            color_value: Color {
                e: (0.73, 0.73, 0.73),
            },
        }),
    });
    let green = Arc::new(Lambertian {
        albedo: Arc::new(SolidColor {
            color_value: Color {
                e: (0.12, 0.45, 0.15),
            },
        }),
    });
    let light = Arc::new(DiffuseLight::new(Color {
        e: (15.0, 15.0, 15.0),
    }));
    objects.add(Arc::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: green.clone(),
    }));
    objects.add(Arc::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: red.clone(),
    }));
    objects.add(Arc::new(XzRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
        mp: light.clone(),
    }));
    objects.add(Arc::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: white.clone(),
    }));
    objects.add(Arc::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));
    objects.add(Arc::new(XyRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));
    objects
}
