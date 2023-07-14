use std::path::Path;

use image::{DynamicImage, RgbImage};

use crate::{
    perlin::Perlin,
    randoms::clamp,
    vec3::{mul_num, Color, Point3},
};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
pub struct SolidColor {
    pub color_value: Color,
}
impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color_value: color }
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}
pub struct CheckerTexture{
    pub odd: Box<dyn Texture>,
    pub even: Box<dyn Texture>,
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.e.0).sin() * (10.0 * p.e.1).sin() * (10.0 * p.e.2).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        mul_num(
            Color { e: (1.0, 1.0, 1.0) },
            0.5 * (1.0 + (10.0 * self.noise.turb(p, 7) + self.scale * p.e.2).sin()),
        )
    }
}
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
