use std::{
    fs::File,
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

use console::style;
use image::{DynamicImage, ImageBuffer, Rgb, RgbImage};
use indicatif::{MultiProgress, ProgressBar};
fn gray_value(r: u8, g: u8, b: u8) -> u8 {
    r.max(g.max(b))
}
fn pixel_gray_value(rgb: &Rgb<u8>) -> i32 {
    (gray_value(rgb.0[0], rgb.0[1], rgb.0[2])) as i32
}
pub fn edgedetect(source: &str, output: &str, thread_num: u32) {
    let source = format!("{}", source); //依需求调整
    let source_path = Path::new(&source);
    let img_: DynamicImage = image::open(source_path).expect("failed");
    let rgb_img: RgbImage = match img_ {
        DynamicImage::ImageRgb8(rgb_img) => rgb_img,
        _ => img_.to_rgb8(),
    };
    let width = rgb_img.width();
    let height = rgb_img.height();
    let source_im = Arc::new(rgb_img);
    let quality = 100;
    let output = format!("output/{}", output);
    let output_path = Path::new(&output);
    let prefix = output_path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    let img2: RgbImage = ImageBuffer::new(width, height);
    let im2 = Arc::new(Mutex::new(img2));
    let main_progress = Arc::new(Mutex::new(MultiProgress::new()));
    let mut handles = vec![];
    for p in 0..thread_num {
        let progress = Arc::new(
            (*main_progress.lock().unwrap())
                .add(ProgressBar::new((height * width / thread_num) as u64)),
        );
        let im_in_thread = source_im.clone();
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
        style(output_path.to_str().unwrap()).yellow()
    );
    let image_now = (*(im2.lock().unwrap())).clone();
    let output_image = image::DynamicImage::ImageRgb8(image_now);
    let mut output_file = File::create(output_path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }
}
