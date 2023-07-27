use crate::{
    camera::{Camera, NewCamMessage},
    hittable::{flipface::FlipFace, sphere::Sphere, HittableList},
    hittable::{
        mybox::MyBox, rect::XyRect, rect::XzRect, rect::YzRect, rotate::RotateY,
        translate::Translate,
    },
    material::{dielectric::Dielectric, diffuselight::DiffuseLight, lambertian::Lambertian},
    texture::solodcolor::SolidColor,
    vec3::{Color, Point3, Vec3},
};
pub fn cornell_box() -> (Color, f64, u32, HittableList, Camera) {
    let mut objects = HittableList::new();
    let red = Lambertian {
        albedo: SolidColor {
            color_value: Color {
                e: (0.65, 0.05, 0.05),
            },
        },
    };
    let white = Lambertian {
        albedo: SolidColor {
            color_value: Color {
                e: (0.73, 0.73, 0.73),
            },
        },
    };
    let green = Lambertian {
        albedo: SolidColor {
            color_value: Color {
                e: (0.12, 0.45, 0.15),
            },
        },
    };
    let light = DiffuseLight::new(Color {
        e: (15.0, 15.0, 15.0),
    });
    let glass = Dielectric { ir: 1.5 };
    objects.add(Box::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: green,
    }));
    objects.add(Box::new(YzRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: red,
    }));

    objects.add(Box::new(FlipFace {
        ptr: XzRect {
            x0: 213.0,
            x1: 343.0,
            z0: 227.0,
            z1: 332.0,
            k: 554.0,
            mp: light,
        },
    }));
    objects.add(Box::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: white.clone(),
    }));
    objects.add(Box::new(XzRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));
    objects.add(Box::new(XyRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));
    /*let aluminum = Metal {
        albedo: Color {
            e: (0.8, 0.85, 0.88),
        },
        fuzz: 0.0,
    };*/
    let box1 = MyBox::new(
        &Point3 { e: (0.0, 0.0, 0.0) },
        &Point3 {
            e: (165.0, 330.0, 165.0),
        },
        white,
    );
    let box1 = RotateY::new(box1, 15.0);
    let box1 = Box::new(Translate {
        offset: Vec3 {
            e: (265.0, 0.0, 295.0),
        },
        ptr: box1,
    });

    objects.add(box1);
    /*let box2 = MyBox::new(
        &Point3 { e: (0.0, 0.0, 0.0) },
        &Point3 {
            e: (165.0, 165.0, 165.0),
        },
        white,
    );
    let box2 = RotateY::new(box2, -18.0);
    let box2 = Box::new(Translate {
        offset: Vec3 {
            e: (130.0, 0.0, 65.0),
        },
        ptr: box2,
    });
    objects.add(box2);*/
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: (190.0, 90.0, 190.0),
        },
        radius: 90.0,
        mat_ptr: glass,
    }));
    let lookfrom: Point3 = Point3 {
        e: (278.0, 278.0, -800.0),
    };
    let lookat: Point3 = Point3 {
        e: (278.0, 278.0, 0.0),
    };
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
