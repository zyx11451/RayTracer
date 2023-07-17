pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod material;
pub mod perlin;
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
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::{fs::File, process::exit};
//use vec3::mul_num;
//use vec3::mul_vec_dot;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::camera::NewCamMessage;
use crate::hittable::Hittable;
use crate::randoms::cornell_box;
//use crate::randoms::cornell_box_smoke;
//use crate::randoms::final_scene;
//use crate::randoms::earth;
//use crate::hittable::Sphere;
//use crate::material::Dielectric;
//use crate::material::Lambertian;
//use crate::material::Metal;
use crate::randoms::random_double;
//use crate::randoms::simple_light;
//use crate::randoms::two_perlin_sphere;
//use crate::randoms::random_scene;
//use crate::randoms::two_spheres;
use crate::ray::write_color;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;

fn ray_color(r: &Ray, background: Color, world: &BvhNode, depth: i32) -> Color {
    let rec: HitRecord;
    if depth <= 0 {
        return Color { e: (0.0, 0.0, 0.0) };
    }
    let w = world.hit(r, 0.001, INFINITY);
    if let Some(..) = w {
        rec = w.unwrap();
        let mut scattered: Ray = Ray {
            orig: (Vec3::new()),
            dir: (Vec3::new()),
            time: 0.0,
        };
        let mut attenuation: Color = Color::new();
        let mut pdf = 1.0;
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
        if rec
            .mat_ptr
            .scatter(r, &rec, &mut attenuation, &mut scattered, &mut pdf)
        {
            emitted
                + attenuation
                    * (rec.mat_ptr.scattering_pdf(r, &rec, &mut scattered) / pdf)
                    * ray_color(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        background
    }
}
fn main() {
    //
    let path = std::path::Path::new("output/book3/test1.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    //Image
    let aspect_ratio: f64 = 1.0;
    let width = 600;
    let height = ((width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let img: RgbImage = ImageBuffer::new(width, height);
    //World
    let background = Color { e: (0.0, 0.0, 0.0) };
    let mut world: HittableList = cornell_box();
    let end = world.objects.len() as u32;
    let bvh = BvhNode::new_nodes(&mut world.objects, 0, end, 0.0, 1.0);
    //Camera
    let lookfrom: Point3 = Point3 {
        e: (278.0, 278.0, -800.0),
    };
    let lookat: Point3 = Point3 {
        e: (278.0, 278.0, 0.0),
    };
    let cam = Camera::new_cam(
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
    );
    //Render
    let thread_num = 20; //必须是图像高度的因数
    let main_progress = Arc::new(Mutex::new(MultiProgress::new()));
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
            for j in (0..height).rev() {
                if j % thread_num == p {
                    for i in 0..width {
                        let mut pixel_color: Color = Color { e: (0.0, 0.0, 0.0) };
                        let mut s = 0;
                        while s < samples_per_pixel {
                            let u =
                                (1.0 * (i as f64) + random_double(0.0, 1.0)) / (width - 1) as f64;
                            let v = (1.0 * ((height - j - 1) as f64) + random_double(0.0, 1.0))
                                / (height - 1) as f64;
                            let r: Ray = cam.get_ray(u, v);
                            pixel_color.add_assign(ray_color(
                                &r,
                                background,
                                bvh_a_in_thread.as_ref(),
                                max_depth,
                            ));
                            s += 1;
                        }
                        let mut img1 = im_in_thread.lock().unwrap();
                        let pixel = (*img1).get_pixel_mut(i, j);
                        *pixel = write_color(pixel_color, samples_per_pixel);
                        progress.inc(1);
                    }
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
