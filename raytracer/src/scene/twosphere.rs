use crate::{
    hittable::sphere::Sphere,
    hittable:: hittable::HittableList,
    material:: lambertian::Lambertian,
    texture::{CheckerTexture, SolidColor},
    vec3::{ Color, Point3},
};
pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();
    let checker = CheckerTexture {
        even: SolidColor::new(Color { e: (0.2, 0.3, 0.1) }),
        odd: SolidColor::new(Color { e: (0.9, 0.9, 0.9) }),
    };
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: (0.0, -10.0, 0.0),
        },
        radius: 10.0,
        mat_ptr: Lambertian { albedo: checker },
    }));
    let checker = CheckerTexture {
        even: SolidColor::new(Color { e: (0.2, 0.3, 0.1) }),
        odd: SolidColor::new(Color { e: (0.9, 0.9, 0.9) }),
    };
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: (0.0, 10.0, 0.0),
        },
        radius: 10.0,
        mat_ptr: Lambertian { albedo: checker },
    }));
    objects
}