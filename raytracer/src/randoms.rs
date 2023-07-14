//随机相关和场景
use crate::{
    bvh::BvhNode,
    hittable::HittableList,
    hittable::{
        ConstantMedium, MovingSphere, MyBox, RotateY, Sphere, Translate, XyRect, XzRect, YzRect,
    },
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
    let checker = CheckerTexture {
        even: SolidColor::new(Color { e: (0.2, 0.3, 0.1) }),
        odd: SolidColor::new(Color { e: (0.9, 0.9, 0.9) }),
    };
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
                        albedo: SolidColor {
                            color_value: albedo_,
                        },
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
        albedo: SolidColor {
            color_value: Color { e: (0.4, 0.2, 0.1) },
        },
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
    let checker = CheckerTexture {
        even: SolidColor::new(Color { e: (0.2, 0.3, 0.1) }),
        odd: SolidColor::new(Color { e: (0.9, 0.9, 0.9) }),
    };
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (0.0, -10.0, 0.0),
        },
        radius: 10.0,
        mat_ptr: Arc::new(Lambertian { albedo: checker }),
    }));
    let checker = CheckerTexture {
        even: SolidColor::new(Color { e: (0.2, 0.3, 0.1) }),
        odd: SolidColor::new(Color { e: (0.9, 0.9, 0.9) }),
    };
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
    let pertext = NoiseTexture {
        noise: Perlin::new(),
        scale: 4.0,
    };
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (0.0, -1000.0, 0.0),
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian { albedo: pertext }),
    }));
    let pertext = NoiseTexture {
        noise: Perlin::new(),
        scale: 4.0,
    };
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
    let earth_texture = ImageTexture::new(path);

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
    let pertext = NoiseTexture {
        noise: Perlin::new(),
        scale: 4.0,
    };
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (0.0, -1000.0, 0.0),
        },
        radius: 1000.0,
        mat_ptr: Arc::new(Lambertian { albedo: pertext }),
    }));
    let pertext = NoiseTexture {
        noise: Perlin::new(),
        scale: 4.0,
    };
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
        albedo: SolidColor {
            color_value: Color {
                e: (0.65, 0.05, 0.05),
            },
        },
    });
    let white = Arc::new(Lambertian {
        albedo: SolidColor {
            color_value: Color {
                e: (0.73, 0.73, 0.73),
            },
        },
    });
    let green = Arc::new(Lambertian {
        albedo: SolidColor {
            color_value: Color {
                e: (0.12, 0.45, 0.15),
            },
        },
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
        mp: green,
    }));
    objects.add(Arc::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: red,
    }));
    objects.add(Arc::new(XzRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
        mp: light,
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
    let box1 =MyBox::new(
        &Point3 { e: (0.0, 0.0, 0.0) },
        &Point3 {
            e: (165.0, 330.0, 165.0),
        },
        white.clone(),
    );
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Arc::new(Translate {
        offset: Vec3 {
            e: (265.0, 0.0, 295.0),
        },
        ptr: box1,
    });

    objects.add(box1);
    let box2 = MyBox::new(
        &Point3 { e: (0.0, 0.0, 0.0) },
        &Point3 {
            e: (165.0, 165.0, 165.0),
        },
        white,
    );
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Arc::new(Translate {
        offset: Vec3 {
            e: (130.0, 0.0, 65.0),
        },
        ptr: box2,
    });
    objects.add(box2);
    objects
}
pub fn cornell_box_smoke() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian {
        albedo: SolidColor {
            color_value: Color {
                e: (0.65, 0.05, 0.05),
            },
        },
    });
    let white = Arc::new(Lambertian {
        albedo: SolidColor {
            color_value: Color {
                e: (0.73, 0.73, 0.73),
            },
        },
    });
    let green = Arc::new(Lambertian {
        albedo: SolidColor {
            color_value: Color {
                e: (0.12, 0.45, 0.15),
            },
        },
    });
    let light = Arc::new(DiffuseLight::new(Color { e: (7.0, 7.0, 7.0) }));
    objects.add(Arc::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: green,
    }));
    objects.add(Arc::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: red,
    }));
    objects.add(Arc::new(XzRect {
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
        mp: light,
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
    let box1 = MyBox::new(
        &Point3 { e: (0.0, 0.0, 0.0) },
        &Point3 {
            e: (165.0, 330.0, 165.0),
        },
        white.clone(),
    );
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Arc::new(Translate {
        offset: Vec3 {
            e: (265.0, 0.0, 295.0),
        },
        ptr: box1,
    });

    objects.add(Arc::new(ConstantMedium::new(
        box1,
        0.01,
        SolidColor::new(Color { e: (0.0, 0.0, 0.0) }),
    )));
    let box2 =MyBox::new(
        &Point3 { e: (0.0, 0.0, 0.0) },
        &Point3 {
            e: (165.0, 165.0, 165.0),
        },
        white,
    );
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Arc::new(Translate {
        offset: Vec3 {
            e: (130.0, 0.0, 65.0),
        },
        ptr: box2,
    });
    objects.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        SolidColor::new(Color { e: (1.0, 1.0, 1.0) }),
    )));
    objects
}
pub fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian {
        albedo: SolidColor::new(Color {
            e: (0.48, 0.83, 0.53),
        }),
    });
    let box_per_side = 20;
    for i in 0..box_per_side {
        for j in 0..box_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Arc::new(MyBox::new(
                &Point3 { e: (x0, y0, z0) },
                &Point3 { e: (x1, y1, z1) },
                ground.clone(),
            )));
        }
    }
    let mut objects = HittableList::new();
    let end = boxes1.objects.len() as u32;
    objects.add(Arc::new(BvhNode::new_nodes(
        boxes1.objects,
        0,
        end,
        0.0,
        1.0,
    )));
    //objects.add(boxes1);
    let light = Arc::new(DiffuseLight::new(Color { e: (7.0, 7.0, 7.0) }));
    objects.add(Arc::new(XzRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        mp: light,
    }));
    let center1_ = Point3 {
        e: (400.0, 400.0, 200.0),
    };
    let center2_ = center1_
        + Vec3 {
            e: (30.0, 0.0, 0.0),
        };
    let moving_sphere_material = Arc::new(Lambertian {
        albedo: SolidColor {
            color_value: Color { e: (0.7, 0.3, 0.1) },
        },
    });
    objects.add(Arc::new(MovingSphere {
        center0: center1_,
        center1: center2_,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat_ptr: moving_sphere_material,
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (260.0, 150.0, 45.0),
        },
        radius: 50.0,
        mat_ptr: Arc::new(Dielectric { ir: 1.5 }),
    }));
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (0.0, 150.0, 145.0),
        },
        radius: 50.0,
        mat_ptr: Arc::new(Metal {
            albedo: Color { e: (0.8, 0.8, 0.9) },
            fuzz: 1.0,
        }),
    }));
    let mut boundary = Arc::new(Sphere {
        center: Point3 {
            e: (360.0, 150.0, 145.0),
        },
        radius: 70.0,
        mat_ptr: Arc::new(Dielectric { ir: 1.5 }),
    });
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        SolidColor::new(Color { e: (0.2, 0.4, 0.9) }),
    )));
    boundary = Arc::new(Sphere {
        center: Point3 { e: (0.0, 0.0, 0.0) },
        radius: 5000.0,
        mat_ptr: Arc::new(Dielectric { ir: 1.5 }),
    });
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        SolidColor::new(Color { e: (1.0, 1.0, 1.0) }),
    )));
    let path = std::path::Path::new("raytracer/src/earthmap.jpg");
    let emat = Arc::new(Lambertian {
        albedo: ImageTexture::new(path),
    });
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (400.0, 200.0, 400.0),
        },
        radius: 100.0,
        mat_ptr: emat,
    }));
    let pertext = NoiseTexture {
        noise: Perlin::new(),
        scale: 0.1,
    };
    objects.add(Arc::new(Sphere {
        center: Point3 {
            e: (220.0, 280.0, 300.0),
        },
        radius: 80.0,
        mat_ptr: Arc::new(Lambertian { albedo: pertext }),
    }));

    let mut boxes2 = HittableList::new();

    let white = Arc::new(Lambertian {
        albedo: SolidColor::new(Color {
            e: (0.73, 0.73, 0.73),
        }),
    });

    let ns = 1000;

    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere {
            center: random_vec(0.0, 165.0),
            radius: 10.0,
            mat_ptr: white.clone(),
        }))
    }
    let end = boxes2.objects.len() as u32;
    objects.add(Arc::new(Translate {
        offset: Vec3 {
            e: (-100.0, 270.0, 395.0),
        },
        ptr:RotateY::new(
            BvhNode::new_nodes(boxes2.objects, 0, end, 0.0, 1.0),
            15.0,
        ),
    }));
    objects
}