use crate::{
    camera::{Camera, NewCamMessage},
    hittable::hittable::HittableList,
    hittable::{rotate::RotateY, translate::Translate},
    loadobj::obj_mtl_load,
    vec3::{Point3, Vec3, Color},
};
pub fn my_world() -> (Color,f64, u32, HittableList, Camera) {
    let mut objects = HittableList::new();
    let battery: String = "1".to_string();
    objects.add(Box::new(RotateY::new(
        Translate {
            offset: Vec3 {
                e: (500.0, 50.0, 0.0),
            },
            ptr: obj_mtl_load(&battery),
        },
        35.0,
    )));
    let lookfrom: Point3 = Point3 {
        e: (400.0, 400.0, -800.0),
    };
    let lookat: Point3 = Point3 { e: (0.0, 0.0, 0.0) };
    let aspect_ratio: f64 = 1.0;
    (
        Color{
            e: (153.0 / 256.0, 204.0 / 256.0, 1.0),
        },
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
