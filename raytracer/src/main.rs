pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod hittable;
pub mod loadobj;
pub mod material;
pub mod pdf;
pub mod perlin;
pub mod randoms;
pub mod ray;
pub mod scene;
pub mod texture;
pub mod vec3;
use console::style;
use hittable::hittable::HitRecord;
use hittable::hittable::HittableList;
use image::Rgb;
use image::{ImageBuffer, RgbImage};
use indicatif::MultiProgress;
use indicatif::ProgressBar;
//use pdf::HittablePdf;
use pdf::MixturePdf;
use pdf::Pdf;
use std::f64::INFINITY;
use std::ops::AddAssign;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::{fs::File, process::exit};

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::camera::NewCamMessage;
use crate::hittable::hittable::Hittable;
use crate::scene::cornellbox::cornell_box;
use crate::scene::cornellboxsmoke::cornell_box_smoke;
use crate::scene::earth::earth;
use crate::scene::finalscene::final_scene;
//use crate::hittable::rect::XzRect;
//use crate::hittable::sphere::Sphere;
//use crate::material::diffuselight::DiffuseLight;
//use crate::scene::cornellbox::cornell_box;
use crate::scene::myworld::my_world;
//use crate::randoms::final_scene;
use crate::randoms::random_double;
use crate::ray::write_color;
use crate::ray::Ray;
use crate::scene::randomscene::random_scene;
use crate::scene::simplelight::simple_light;
use crate::scene::twoperlinsphere::two_perlin_sphere;
use crate::scene::twosphere::two_spheres;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;

fn ray_color(
    r: &Ray,
    lights: &HittableList,
    background: Color,
    world: &BvhNode,
    depth: i32,
) -> Color {
    let rec: HitRecord;
    if depth <= 0 {
        return Color { e: (0.0, 0.0, 0.0) };
    }
    let w = world.hit(r, 0.001, INFINITY);
    if let Some(..) = w {
        rec = w.unwrap();
        let mut scattered: Ray;
        let emitted = rec.mat_ptr.emitted(r, &rec, rec.u, rec.v, &rec.p);
        let k = rec.mat_ptr.scatter(r, &rec);
        if let Some(..) = k {
            let srec = k.unwrap();
            if srec.is_specular {
                return srec.attenuation
                    * ray_color(&srec.specular_ray, lights, background, world, depth - 1);
            }
            let p2_ = srec.pdf_ptr.unwrap();
            let p = {
                MixturePdf {
                    //切换的时候要改一下
                    /*p1: &HittablePdf {
                        o: rec.p,
                        ptr: lights,
                    },*/
                    p1: p2_.as_ref(),
                    p2: p2_.as_ref(),
                }
            };
            scattered = Ray {
                orig: rec.p,
                dir: p.generate(),
                time: r.time,
            };
            let pdf_val = p.value(&scattered.dir);
            emitted
                + srec.attenuation
                    * (rec.mat_ptr.scattering_pdf(r, &rec, &mut scattered))
                    * (ray_color(&scattered, lights, background, world, depth - 1) / pdf_val)
        } else {
            emitted
        }
    } else {
        background
    }
}
fn gray_value(r: u8, g: u8, b: u8) -> u8 {
    r.max(g.max(b))
}
fn pixel_gray_value(rgb: &Rgb<u8>) -> i32 {
    (gray_value(rgb.0[0], rgb.0[1], rgb.0[2])) as i32
}
fn main() {
    //
    let test_cornell_box = true;
    if test_cornell_box {
        let path = std::path::Path::new("output/book3/work.jpg");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
        let quality = 100;
        let samples_per_pixel = 100; //1000
        let max_depth = 50;
        let (background,aspect_ratio,width, mut world, cam) = random_scene();
        let height = ((width as f64) / aspect_ratio) as u32;
        let lights = HittableList::new();
        let img: RgbImage = ImageBuffer::new(width, height);
        let end = world.objects.len() as u32;
        let bvh = BvhNode::new_nodes(&mut world.objects, 0, end, 0.0, 1.0);
        let thread_num = 20; //必须是图像高度的因数
        let main_progress = Arc::new(Mutex::new(MultiProgress::new()));
        let bvh_a = Arc::new(bvh);
        let im = Arc::new(Mutex::new(img));
        let lights = Arc::new(lights);
        let mut handles = vec![];
        for p in 0..thread_num {
            let progress = Arc::new(
                (*main_progress.lock().unwrap())
                    .add(ProgressBar::new((height * width / thread_num) as u64)),
            );
            //let b_in_thread = b.clone();
            let bvh_a_in_thread = bvh_a.clone();
            let im_in_thread = im.clone();
            let lights_in_thread = lights.clone();
            let each_thread = thread::spawn(move || {
                for j in (0..height).rev() {
                    if j % thread_num == p {
                        for i in 0..width {
                            let mut pixel_color: Color = Color { e: (0.0, 0.0, 0.0) };
                            let mut s = 0;
                            while s < samples_per_pixel {
                                let u = (1.0 * (i as f64) + random_double(0.0, 1.0))
                                    / (width - 1) as f64;
                                let v = (1.0 * ((height - j - 1) as f64) + random_double(0.0, 1.0))
                                    / (height - 1) as f64;
                                let r: Ray = cam.get_ray(u, v);
                                pixel_color.add_assign(ray_color(
                                    &r,
                                    lights_in_thread.as_ref(),
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
    let path = std::path::Path::new("output/book3/test_ran_scene.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    //Image
    //random_scene_进行边缘检测的测试
    let quality = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;
    

    //World
    let (background,aspect_ratio,width, mut world, cam)= random_scene();
    let height = ((width as f64) / aspect_ratio) as u32;
    let img: RgbImage = ImageBuffer::new(width, height);
    let lights = HittableList::new();
    let end = world.objects.len() as u32;
    let bvh = BvhNode::new_nodes(&mut world.objects, 0, end, 0.0, 1.0);
    //Render
    let thread_num = 20; //必须是图像高度的因数
    let main_progress = Arc::new(Mutex::new(MultiProgress::new()));
    let bvh_a = Arc::new(bvh);
    let im = Arc::new(Mutex::new(img));
    let lights = Arc::new(lights);
    let mut handles = vec![];
    for p in 0..thread_num {
        let progress = Arc::new(
            (*main_progress.lock().unwrap())
                .add(ProgressBar::new((height * width / thread_num) as u64)),
        );
        //let b_in_thread = b.clone();
        let bvh_a_in_thread = bvh_a.clone();
        let im_in_thread = im.clone();
        let lights_in_thread = lights.clone();
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
                                lights_in_thread.as_ref(),
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
    //边缘检测
    let path = std::path::Path::new("output/book3/test_ran_scene_e.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    let im1 = Arc::new((*(im.lock().unwrap())).clone());
    let img2: RgbImage = ImageBuffer::new(width, height);
    let im2 = Arc::new(Mutex::new(img2));
    let main_progress = Arc::new(Mutex::new(MultiProgress::new()));
    let mut handles = vec![];
    for p in 0..thread_num {
        let progress = Arc::new(
            (*main_progress.lock().unwrap())
                .add(ProgressBar::new((height * width / thread_num) as u64)),
        );
        let im_in_thread = im1.clone();
        let im2_in_thread = im2.clone();
        let gx = [-1, 0, 1, -2, -0, 2, -1, 0, 1];
        let gy = [-1, -2, -1, 0, 0, 0, 1, 2, 1];
        let each_thread = thread::spawn(move || {
            for j in (0..height).rev() {
                if j % thread_num == p {
                    for i in 0..width {
                        if j != 0 && i != 0 && j != height - 1 && i != width - 1 {
                            let pixel0 = im_in_thread.get_pixel(i - 1, j - 1);
                            let pixel1 = im_in_thread.get_pixel(i - 1, j);
                            let pixel2 = im_in_thread.get_pixel(i - 1, j + 1);
                            let pixel3 = im_in_thread.get_pixel(i, j - 1);
                            let pixel4 = im_in_thread.get_pixel(i, j);
                            let pixel5 = im_in_thread.get_pixel(i, j + 1);
                            let pixel6 = im_in_thread.get_pixel(i + 1, j - 1);
                            let pixel7 = im_in_thread.get_pixel(i + 1, j);
                            let pixel8 = im_in_thread.get_pixel(i + 1, j + 1);
                            let gxx = pixel_gray_value(pixel0) * gx[0]
                                + pixel_gray_value(pixel1) * gx[1]
                                + pixel_gray_value(pixel2) * gx[2]
                                + pixel_gray_value(pixel3) * gx[3]
                                + pixel_gray_value(pixel4) * gx[4]
                                + pixel_gray_value(pixel5) * gx[5]
                                + pixel_gray_value(pixel6) * gx[6]
                                + pixel_gray_value(pixel7) * gx[7]
                                + pixel_gray_value(pixel8) * gx[8];
                            let gyy = pixel_gray_value(pixel0) * gy[0]
                                + pixel_gray_value(pixel1) * gy[1]
                                + pixel_gray_value(pixel2) * gy[2]
                                + pixel_gray_value(pixel3) * gy[3]
                                + pixel_gray_value(pixel4) * gy[4]
                                + pixel_gray_value(pixel5) * gy[5]
                                + pixel_gray_value(pixel6) * gy[6]
                                + pixel_gray_value(pixel7) * gy[7]
                                + pixel_gray_value(pixel8) * gy[8];
                            let g = ((gxx * gxx + gyy * gyy) as f64).sqrt();
                            let mut img2 = im2_in_thread.lock().unwrap();
                            let pixel = (*img2).get_pixel_mut(i, j);
                            if g > 64.0 {
                                *pixel = image::Rgb([0_u8, 0_u8, 0_u8])
                            } else {
                                *pixel = *pixel4;
                            }

                            progress.inc(1);
                        } else {
                            let mut img2 = im2_in_thread.lock().unwrap();
                            let pixel = (*img2).get_pixel_mut(i, j);
                            *pixel = *(im_in_thread.get_pixel(i, j));
                            progress.inc(1);
                        }
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
    let image_now = (*(im2.lock().unwrap())).clone();
    let output_image = image::DynamicImage::ImageRgb8(image_now);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }
    exit(0);
}
