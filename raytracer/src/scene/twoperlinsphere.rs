use crate::{
    hittable::sphere::Sphere,
    hittable:: hittable::HittableList,
    material:: lambertian::Lambertian,
    perlin::Perlin,
    texture:: NoiseTexture,
    vec3:: Point3,
};
pub fn two_perlin_sphere() -> HittableList {
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
    objects
}