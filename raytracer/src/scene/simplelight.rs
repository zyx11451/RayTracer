use crate::{
    hittable::sphere::Sphere,
    hittable::rect::XyRect,
    hittable:: hittable::HittableList,
    material::{ diffuselight::DiffuseLight, lambertian::Lambertian},
    perlin::Perlin,
    texture:: NoiseTexture,
    vec3::{ Color, Point3},
};
pub fn simple_light() -> HittableList {
    let mut objects = HittableList::new();
    let pertext = NoiseTexture {
        noise: Perlin::new(),
        scale: 4.0,
    };
    objects.add(Box::new(Sphere {
        center: Point3 {
            e: (0.0, -1000.0, 0.0),
        },
        radius: 1000.0,
        mat_ptr: Lambertian { albedo: pertext },
    }));
    let pertext = NoiseTexture {
        noise: Perlin::new(),
        scale: 4.0,
    };
    objects.add(Box::new(Sphere {
        center: Point3 { e: (0.0, 2.0, 0.0) },
        radius: 2.0,
        mat_ptr: Lambertian { albedo: pertext },
    }));
    let difflight = DiffuseLight::new(Color { e: (4.0, 4.0, 4.0) });
    objects.add(Box::new(XyRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        mp: difflight,
    }));
    let difflight = DiffuseLight::new(Color { e: (4.0, 4.0, 4.0) });
    objects.add(Box::new(Sphere {
        center: Point3 { e: (0.0, 7.0, 0.0) },
        radius: 2.0,
        mat_ptr: difflight,
    }));
    objects
}