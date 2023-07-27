use std::path::Path;

use image::{DynamicImage, RgbImage};

use crate::texture::Texture;
use crate::{
    randoms::clamp,
    vec3::{Color, Point3},
};
#[derive(Clone)]
pub struct ImageTexture {
    pub img: RgbImage,
    pub width: u32,
    pub height: u32,
}
impl ImageTexture {
    pub fn new(data: &Path) -> Self {
        let img_: DynamicImage = image::open(data).expect("failed");
        let rgb_img: RgbImage = match img_ {
            DynamicImage::ImageRgb8(rgb_img) => rgb_img,
            _ => img_.to_rgb8(),
        };
        Self {
            img: rgb_img.clone(),
            width: rgb_img.width(),
            height: rgb_img.height(),
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        let uu = clamp(u, 0.0, 1.0);
        let vv = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (uu * (self.width as f64)) as u32;
        let mut j = (vv * (self.height as f64)) as u32;
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i, j);
        Color {
            e: (
                color_scale * pixel.0[0] as f64,
                color_scale * pixel.0[1] as f64,
                color_scale * pixel.0[2] as f64,
            ),
        }
    }
}
