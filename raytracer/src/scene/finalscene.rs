use crate::camera::{Camera, NewCamMessage};
//这是book2的最后一张图
use crate::randoms::{random_double, random_vec};
use crate::{
    bvh::BvhNode,
    hittable::movingsphere::MovingSphere,
    hittable::sphere::Sphere,
    hittable::{
        constantmedium::ConstantMedium, mybox::MyBox, rect::XzRect, rotate::RotateY,
        translate::Translate,
    },
    hittable::{flipface::FlipFace, hittable::HittableList},
    material::{
        dielectric::Dielectric, diffuselight::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    perlin::Perlin,
    texture::{imagetexture::ImageTexture, noisetexture::NoiseTexture, solodcolor::SolidColor},
    vec3::{Color, Point3, Vec3},
};
pub fn final_scene() -> (Color, f64, u32, HittableList, Camera) {
    let mut boxes1 = HittableList::new();
    let ground = Lambertian {
        albedo: SolidColor::new(Color {
            e: (0.48, 0.83, 0.53),
        }),
    };
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
            boxes1.add(Box::new(MyBox::new(
                &Point3 { e: (x0, y0, z0) },
                &Point3 { e: (x1, y1, z1) },
                ground.clone(),
            )));
        }
    }
    let mut objects = HittableList::new();
    let end = boxes1.objects.len() as u32;
    objects.add(Box::new(BvhNode::new_nodes(
        &mut boxes1.objects,
        0,
        end,
        0.0,
        1.0,
    )));
    let light = DiffuseLight::new(Color { e: (7.0, 7.0, 7.0) });
    objects.add(Box::new(FlipFace {
        ptr: XzRect {
            x0: 123.0,
            x1: 423.0,
            z0: 147.0,
            z1: 412.0,
            k: 554.0,
            mp: light,
        },
    }));
    let center1_ = Point3 {
        e: (400.0, 400.0, 200.0),
    };
    let center2_ = center1_
        + Vec3 {
            e: (30.0, 0.0, 0.0),
        };
    let moving_sphere_material = Lambertian {
        albedo: SolidColor {
            color_value: Color { e: (0.7, 0.3, 0.1) },
        },
    };
    objects.add(Box::new(MovingSphere {
        center0: center1_,
        center1: center2_,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat_ptr: moving_sphere_material,
    }));
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: (260.0, 150.0, 45.0),
        },
        radius: 50.0,
        mat_ptr: Dielectric { ir: 1.5 },
    }));
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: (0.0, 150.0, 145.0),
        },
        radius: 50.0,
        mat_ptr: Metal {
            albedo: Color { e: (0.8, 0.8, 0.9) },
            fuzz: 1.0,
        },
    }));
    let boundary = Box::new(Sphere {
        center: Point3 {
            e: (360.0, 150.0, 145.0),
        },
        radius: 70.0,
        mat_ptr: Dielectric { ir: 1.5 },
    });
    objects.add(boundary);
    let mut boundary = Sphere {
        center: Point3 {
            e: (360.0, 150.0, 145.0),
        },
        radius: 70.0,
        mat_ptr: Dielectric { ir: 1.5 },
    };
    objects.add(Box::new(ConstantMedium::new(
        boundary,
        0.2,
        SolidColor::new(Color { e: (0.2, 0.4, 0.9) }),
    )));
    boundary = Sphere {
        center: Point3 { e: (0.0, 0.0, 0.0) },
        radius: 5000.0,
        mat_ptr: Dielectric { ir: 1.5 },
    };
    objects.add(Box::new(ConstantMedium::new(
        boundary,
        0.0001,
        SolidColor::new(Color { e: (1.0, 1.0, 1.0) }),
    )));
    let path = std::path::Path::new("raytracer/src/earthmap.jpg");
    let emat = Lambertian {
        albedo: ImageTexture::new(path),
    };
    objects.add(Box::new(Sphere {
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
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: (220.0, 280.0, 300.0),
        },
        radius: 80.0,
        mat_ptr: Lambertian { albedo: pertext },
    }));

    let mut boxes2 = HittableList::new();

    let ns = 1000;

    for _j in 0..ns {
        let white = Lambertian {
            albedo: SolidColor::new(Color {
                e: (0.73, 0.73, 0.73),
            }),
        };

        boxes2.add(Box::new(Sphere {
            center: random_vec(0.0, 165.0),
            radius: 10.0,
            mat_ptr: white,
        }))
    }
    let end = boxes2.objects.len() as u32;
    objects.add(Box::new(Translate {
        offset: Vec3 {
            e: (-100.0, 270.0, 395.0),
        },
        ptr: RotateY::new(
            BvhNode::new_nodes(&mut boxes2.objects, 0, end, 0.0, 1.0),
            15.0,
        ),
    }));
    let lookfrom: Point3 = Point3 {
        e: (478.0, 278.0, -600.0),
    };
    let lookat: Point3 = Point3 { e: (278.0, 278.0, 0.0) };
    let aspect_ratio: f64 = 1.0;
    (
        Color { e: (0.0, 0.0, 0.0) },
        1.0,
        600,
        objects,
        Camera::new_cam(
            lookfrom,
            lookat,
            Vec3 { e: (0.0, 1.0, 0.0) },
            NewCamMessage {
                vfov: 40.0,
                _aspect_ratio: aspect_ratio,
                aperture: 0.0,
                focus_dist: 10.0,
                _time0: 0.0,
                _time1: 1.0,
            },
        ),
    )
}
