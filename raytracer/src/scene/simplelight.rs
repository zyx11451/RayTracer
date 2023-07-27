use crate::{
    camera::{Camera, NewCamMessage},
    hittable::rect::XyRect,
    hittable::sphere::Sphere,
    hittable::HittableList,
    material::{diffuselight::DiffuseLight, lambertian::Lambertian},
    perlin::Perlin,
    texture::noisetexture::NoiseTexture,
    vec3::{Color, Point3, Vec3},
};
pub fn simple_light() -> (Color, f64, u32, HittableList, Camera) {
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
    let lookfrom: Point3 = Point3 {
        e: (26.0, 3.0, 6.0),
    };
    let lookat: Point3 = Point3 { e: (0.0, 2.0, 0.0) };
    let aspect_ratio: f64 = 16.0 / 9.0;
    (
        Color { e: (0.0, 0.0, 0.0) },
        aspect_ratio,
        1600,
        objects,
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
