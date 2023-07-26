use crate::{
    hittable::{
        rotate::RotateY, translate::Translate,
    },
    hittable:: hittable::HittableList,
    loadobj::obj_mtl_load,
    vec3::Vec3,
};
pub fn my_world() -> HittableList {
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
    objects
}