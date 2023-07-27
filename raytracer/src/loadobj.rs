use std::path::Path;

use tobj::{load_obj, LoadOptions};

use crate::{
    bvh::BvhNode,
    hittable::hittable::{Hittable, HittableList},
    hittable::triangle::Triangle,
    material::lambertian::Lambertian,
    texture::objloadingtexture::ObjLoadingTexture,
    vec3::Point3,
};
pub fn obj_mtl_load(name: &String) -> HittableList {
    let mut my_model = HittableList::new();
    let path = format!("raytracer/objects/{}", name);
    let project = format!("{}/{}.obj", path, name);
    let (models, materials) = load_obj(
        project,
        &LoadOptions {
            single_index: (false),
            triangulate: (true),
            ignore_points: (true),
            ignore_lines: (true),
        },
    )
    .expect("failed");
    let m = materials.unwrap();
    let mut textures: Vec<ObjLoadingTexture> = Vec::new();

    for mat in m {
        if let Some(texture_name) = mat.diffuse_texture {
            let path = format!("raytracer/objects/{}", name);
            let texture_path = format!("{}/{}", path, texture_name);
            textures.push(ObjLoadingTexture::new(Path::new(&texture_path)));
        } else {
            if let Some(solid_texture) = mat.diffuse {
                textures.push(ObjLoadingTexture::new_solid_color((
                    solid_texture[0],
                    solid_texture[1],
                    solid_texture[2],
                )))
            } else {
                textures.push(ObjLoadingTexture::new_solid_color((0.0, 0.0, 0.0)));
            }
        }
    }
    for mo in models {
        let pos = mo.mesh.positions;
        let ind = mo.mesh.indices;
        let tex = mo.mesh.texcoords;
        let tex_ind = mo.mesh.texcoord_indices;
        let mut points = Vec::new();
        let mut hittable_mo: Vec<Box<dyn Hittable>> = Vec::new();
        for i in 0..pos.len() / 3 {
            points.push(Point3 {
                e: (pos[i * 3], pos[i * 3 + 1], pos[i * 3 + 2]),
            });
        }
        let texture_mo = textures[mo.mesh.material_id.unwrap()].clone();
        let mut uv = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)];
        for i in 0..ind.len() / 3 {
            for j in 0..3 {
                let index = tex_ind[i * 3 + j] as usize;
                uv[j] = (tex[index << 1], tex[(index << 1) | 1]);
            }
            hittable_mo.push(Box::new(Triangle::new(
                points[ind[i * 3] as usize],
                points[ind[i * 3 + 1] as usize],
                points[ind[i * 3 + 2] as usize],
                Lambertian {
                    albedo: texture_mo.clone(),
                },
                uv[0],
                uv[1],
                uv[2],
            )))
        }
        let end = hittable_mo.len() as u32;
        my_model.add(Box::new(BvhNode::new_nodes(
            &mut hittable_mo,
            0,
            end,
            0.0,
            0.0,
        )));
    }
    let end = my_model.objects.len() as u32;
    let bvh = BvhNode::new_nodes(&mut my_model.objects, 0, end, 0.0, 0.0);
    let mut ans = HittableList::new();
    ans.add(Box::new(bvh));
    ans
}
