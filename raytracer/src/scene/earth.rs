use crate::{
    hittable::sphere::Sphere,
    hittable:: hittable::HittableList,
    material:: lambertian::Lambertian,
    texture:: ImageTexture,
    vec3:: Point3,
};
pub fn earth() -> HittableList {
    let mut objects = HittableList::new();
    let path = std::path::Path::new("raytracer/src/earthmap.jpg");
    let earth_texture = ImageTexture::new(path);

    let earth_surface = Lambertian {
        albedo: earth_texture,
    };
    objects.add(Box::new(Sphere {
        center: Point3 { e: (0.0, 0.0, 0.0) },
        radius: 2.0,
        mat_ptr: earth_surface,
    }));
    objects
}