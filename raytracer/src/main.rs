pub mod camera;
pub mod hittable;
pub mod randoms;
pub mod ray;
pub mod vec3;
use console::style;
use hittable::HitRecord;
use hittable::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use randoms::random_in_semi_sphere;
use std::f64::INFINITY;
use std::ops::AddAssign;
use std::{fs::File, process::exit};
use vec3::mul_num;
//use vec3::mul_vec_dot;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable::Sphere;
use crate::randoms::random_double;
use crate::ray::write_color;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::vec3::Vec3;

fn ray_color(r: Ray, world: &mut HittableList, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord::new();
    if depth <= 0 {
        return Color { e: (0.0, 0.0, 0.0) };
    }
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target: Point3 = rec.p + rec.normal + random_in_semi_sphere(rec.normal);
        mul_num(
            ray_color(
                Ray {
                    orig: (rec.p),
                    dir: (target - rec.p),
                },
                world,
                depth - 1,
            ),
            0.5,
        )
    } else {
        let unit_direction: Vec3 = r.dir.unit_vector();
        let t: f64 = 0.5 * (unit_direction.e.1 + 1.0);
        mul_num(Color { e: (1.0, 1.0, 1.0) }, 1.0 - t) + mul_num(Color { e: (0.5, 0.7, 1.0) }, t)
    }
}
fn main() {
    //
    let path = std::path::Path::new("output/book1/image10.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width = 400;
    let height = ((width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    //World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere {
        center: Point3 {
            e: (0.0, 0.0, -1.0),
        },
        radius: 0.5,
    }));
    world.add(Box::new(Sphere {
        center: Point3 {
            e: (0.0, -100.5, -1.0),
        },
        radius: 100.0,
    }));
    //Camera
    let cam = Camera::new();
    //Render
    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);
            let mut pixel_color: Color = Color { e: (0.0, 0.0, 0.0) };
            let mut s = 0;
            while s < samples_per_pixel {
                let u = (1.0 * (i as f64) + random_double(0.0, 1.0)) / (width - 1) as f64;
                let v = (1.0 * ((height - j - 1) as f64) + random_double(0.0, 1.0))
                    / (height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                pixel_color.add_assign(ray_color(r, &mut world, max_depth));
                s += 1;
            }
            *pixel = write_color(pixel_color, samples_per_pixel);
            progress.inc(1);
        }
    }
    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }
    exit(0);
}
