use crate::randoms::{random_double, random_vec};
use crate::{
    camera::{Camera, NewCamMessage},
    hittable::sphere::Sphere,
    hittable::HittableList,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    texture::{checkertexture::CheckerTexture, solodcolor::SolidColor},
    vec3::{Color, Point3, Vec3},
};

pub fn random_scene() -> (Color, f64, u32, HittableList, Camera) {
    let mut world = HittableList::new();
    let checker = CheckerTexture {
        even: SolidColor::new(Color { e: (0.2, 0.3, 0.1) }),
        odd: SolidColor::new(Color { e: (0.9, 0.9, 0.9) }),
    };
    let ground_material = Lambertian { albedo: checker };
    world.add(Box::new(Sphere {
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
                    /*let center2 = center_
                    + Vec3 {
                        e: (0.0, random_double(0.0, 0.5), 0.0),
                    };*/
                    let sphere_material = Lambertian {
                        albedo: SolidColor {
                            color_value: albedo_,
                        },
                    };
                    world.add(Box::new(Sphere {
                        center: center_,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    let albedo_ = random_vec(0.5, 1.0);
                    let fuzz_ = random_double(0.0, 0.5);
                    let sphere_material = Metal {
                        albedo: albedo_,
                        fuzz: fuzz_,
                    };
                    world.add(Box::new(Sphere {
                        center: center_,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                } else {
                    let sphere_material = Dielectric { ir: 1.5 };
                    world.add(Box::new(Sphere {
                        center: center_,
                        radius: 0.2,
                        mat_ptr: sphere_material,
                    }));
                }
            }
        }
    }
    let material1 = Dielectric { ir: 1.5 };
    world.add(Box::new(Sphere {
        center: Point3 { e: (0.0, 1.0, 0.0) },
        radius: 1.0,
        mat_ptr: material1,
    }));
    let material2 = Lambertian {
        albedo: SolidColor {
            color_value: Color { e: (0.4, 0.2, 0.1) },
        },
    };
    world.add(Box::new(Sphere {
        center: Point3 {
            e: (-4.0, 1.0, 0.0),
        },
        radius: 1.0,
        mat_ptr: material2,
    }));
    let material3 = Metal {
        albedo: Color { e: (0.7, 0.6, 0.5) },
        fuzz: 0.0,
    };
    world.add(Box::new(Sphere {
        center: Point3 { e: (4.0, 1.0, 0.0) },
        radius: 1.0,
        mat_ptr: material3,
    }));
    let lookfrom: Point3 = Point3 {
        e: (13.0, 2.0, 3.0),
    };
    let lookat: Point3 = Point3 { e: (0.0, 0.0, 0.0) };
    let aspect_ratio: f64 = 16.0 / 9.0;
    (
        Color { e: (0.7, 0.8, 1.0) },
        aspect_ratio,
        1600,
        world,
        Camera::new_cam(
            lookfrom,
            lookat,
            Vec3 { e: (0.0, 1.0, 0.0) },
            NewCamMessage {
                vfov: 20.0,
                _aspect_ratio: aspect_ratio,
                aperture: 0.0,
                focus_dist: 10.0,
                _time0: 0.0,
                _time1: 1.0,
            },
        ),
    )
}
