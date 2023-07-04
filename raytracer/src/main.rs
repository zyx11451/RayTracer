pub mod vec3;
pub mod ray;
use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use vec3::mul_num;
use std::{fs::File, process::exit};

use crate::vec3::Vec3;
use crate::ray::write_color;
use crate::vec3::Color;
use crate::vec3::Point3;
use crate::ray::Ray;

fn ray_color(r:Ray)->Color{
    let unit_direction:Vec3 = r.dir.unit_vector();
    let t:f64 =0.5*(unit_direction.e.1+1.0);
    mul_num(Color{e:(1.0,1.0,1.0)}, 1.0-t)+ mul_num(Color{e:(0.5,0.7,1.0)}, t)
}
fn main() {
    //
    let path = std::path::Path::new("output/book1/image2.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    //Image
    let aspect_ratio:f64=16.0/9.0;
    let width = 400;
    let height = ((width as f64)/aspect_ratio ) as u32;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    //Camera
    let  viewport_height = 2.0;
    let  viewport_width = aspect_ratio * viewport_height;
    let  focal_length = 1.0;

    let  origin = Point3{e:(0.0, 0.0, 0.0)};
    let  horizontal = Vec3{e:(viewport_width, 0.0, 0.0)};
    let  vertical = Vec3{e:(0.0, viewport_height, 0.0)};
    let  lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{e:(0.0, 0.0, focal_length)};
    //Render
    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);
            let  u= (1.0* i as f64 )/ (width-1) as f64;
            let  v=(1.0 * (height-j-1) as f64) /(height-1) as f64;
            let r:Ray= Ray { orig: origin, 
                             dir:  lower_left_corner+mul_num(horizontal, u)+mul_num(vertical, v)-origin};
            let pixel_color:Color=ray_color(r);
            *pixel = write_color(pixel_color);
        }
        progress.inc(1);
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
