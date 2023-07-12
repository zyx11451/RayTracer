pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod material;
pub mod randoms;
pub mod ray;
pub mod texture;
pub mod vec3;
use console::style;
use hittable::HitRecord;
use hittable::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::MultiProgress;
use indicatif::ProgressBar;
//use std::f64::consts::PI;
use std::f64::INFINITY;
use std::ops::AddAssign;
//use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::{fs::File, process::exit};
use vec3::mul_num;
//use vec3::mul_vec_dot;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::camera::NewCamMessage;
use crate::hittable::Hittable;
//use crate::hittable::Sphere;
//use crate::material::Dielectric;
//use crate::material::Lambertian;
//use crate::material::Metal;
use crate::randoms::random_double;
use crate::randoms::random_scene;
use crate::ray::write_color;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;

/*fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if depth <= 0 {
        return Color { e: (0.0, 0.0, 0.0) };
    }
    if world.hit(r, 0.001,  INFINITY, &mut rec) {
        let mut scattered: Ray = Ray {
            orig: (Vec3::new()),
            dir: (Vec3::new()),
            time: 0.0,
        };
        let mut attenuation: Color = Color::new();
        if rec
            .mat_ptr
            .scatter(r, rec.clone(), &mut attenuation, &mut scattered)
        {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color { e: (0.0, 0.0, 0.0) }
        }
    } else {
        let unit_direction: Vec3 = r.dir.unit_vector();
        let t: f64 = 0.5 * (unit_direction.e.1 + 1.0);
        mul_num(Color { e: (1.0, 1.0, 1.0) }, 1.0 - t) + mul_num(Color { e: (0.5, 0.7, 1.0) }, t)
    }
}*/
fn ray_color(r: &Ray, world: &BvhNode, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if depth <= 0 {
        return Color { e: (0.0, 0.0, 0.0) };
    }
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Ray {
            orig: (Vec3::new()),
            dir: (Vec3::new()),
            time: 0.0,
        };
        let mut attenuation: Color = Color::new();
        if rec
            .mat_ptr
            .scatter(r, rec.clone(), &mut attenuation, &mut scattered)
        {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color { e: (0.0, 0.0, 0.0) }
        }
    } else {
        let unit_direction: Vec3 = r.dir.unit_vector();
        let t: f64 = 0.5 * (unit_direction.e.1 + 1.0);
        mul_num(Color { e: (1.0, 1.0, 1.0) }, 1.0 - t) + mul_num(Color { e: (0.5, 0.7, 1.0) }, t)
    }
}
fn main() {
    //
    let path = std::path::Path::new("output/book2/test.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width = 400;
    let height = ((width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let img: RgbImage = ImageBuffer::new(width, height);
    //World
    let world: HittableList = random_scene();
    let end = world.objects.len() as u32;
    let bvh = BvhNode::new_nodes(world.objects, 0, end, 0.0, 1.0);
    //Camera
    let lookfrom: Point3 = Point3 {
        e: (13.0, 2.0, 3.0),
    };
    let lookat: Point3 = Point3 { e: (0.0, 0.0, 0.0) };
    let cam = Camera::new_cam(
        lookfrom,
        lookat,
        Vec3 { e: (0.0, 1.0, 0.0) },
        NewCamMessage {
            vfov: 20.0,
            _aspect_ratio: aspect_ratio,
            aperture: 0.1,
            focus_dist: 10.0,
            _time0: 0.0,
            _time1: 1.0,
        },
    );
    //Render
    let thread_num = 15; //必须是图像高度的因数
    let main_progress = Arc::new(Mutex::new(MultiProgress::new()));
    let thread_height = height / thread_num;
    //let b = Arc::new(world);
    let bvh_a = Arc::new(bvh);
    let im = Arc::new(Mutex::new(img));
    let mut handles = vec![];
    for p in 0..thread_num {
        let progress = Arc::new(
            (*main_progress.lock().unwrap())
                .add(ProgressBar::new((height * width / thread_num) as u64)),
        );
        //let b_in_thread = b.clone();
        let bvh_a_in_thread = bvh_a.clone();
        let im_in_thread = im.clone();
        let each_thread = thread::spawn(move || {
            let max_h = ((p + 1) * thread_height).min(height);
            for j in (p * thread_height..max_h).rev() {
                for i in 0..width {
                    let mut pixel_color: Color = Color { e: (0.0, 0.0, 0.0) };
                    let mut s = 0;
                    while s < samples_per_pixel {
                        let u = (1.0 * (i as f64) + random_double(0.0, 1.0)) / (width - 1) as f64;
                        let v = (1.0 * ((height - j - 1) as f64) + random_double(0.0, 1.0))
                            / (height - 1) as f64;
                        let r: Ray = cam.get_ray(u, v);
                        pixel_color.add_assign(ray_color(&r, bvh_a_in_thread.as_ref(), max_depth));
                        s += 1;
                    }
                    let mut img1 = im_in_thread.lock().unwrap();
                    let pixel = (*img1).get_pixel_mut(i, j);
                    *pixel = write_color(pixel_color, samples_per_pixel);
                    progress.inc(1);
                }
            }
            progress.finish();
        });
        handles.push(each_thread);
    }

    main_progress.lock().unwrap().join().unwrap();

    for th in handles {
        th.join().unwrap();
    }
    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let image_now = (*(im.lock().unwrap())).clone();
    let output_image = image::DynamicImage::ImageRgb8(image_now);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }
    exit(0);
}
