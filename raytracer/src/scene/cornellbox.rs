use crate::{
    hittable::{
         mybox::MyBox, rect::XyRect, rect::XzRect, rect::YzRect,
        rotate::RotateY, translate::Translate,
    },
    hittable::{sphere::Sphere,flipface::FlipFace, hittable::HittableList},
    material::{dielectric::Dielectric, diffuselight::DiffuseLight, lambertian::Lambertian},
    texture::SolidColor,
    vec3::{ Color, Point3, Vec3},
};
pub fn cornell_box() -> HittableList {
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
        mp: red.clone(),
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
        white.clone(),
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
    objects
}