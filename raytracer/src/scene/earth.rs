use crate::{
    camera::{Camera, NewCamMessage},
    hittable::sphere::Sphere,
    hittable::HittableList,
    material::lambertian::Lambertian,
    texture::imagetexture::ImageTexture,
    vec3::{Color, Point3, Vec3},
};
pub fn earth() -> (Color, f64, u32, HittableList, Camera) {
    let mut objects = HittableList::new();
    let path = std::path::Path::new("raytracer/src/sourcepictures/earthmap.jpg");
    let earth_texture = ImageTexture::new(path);

    let earth_surface = Lambertian {
        albedo: earth_texture,
    };
    objects.add(Box::new(Sphere {
        center: Point3 { e: (0.0, 0.0, 0.0) },
        radius: 2.0,
        mat_ptr: earth_surface,
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
