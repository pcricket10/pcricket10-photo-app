#![allow(dead_code)]

use crate::image::Image;
use image::{imageops, RgbImage};
pub struct Filter;

impl Filter {
    pub fn blur(image: &mut Image, sigma: f32) -> RgbImage {
        imageops::blur(&image.to_rgb_image(), sigma)
    }

    pub fn emphasize_blue(image: &mut Image, factor: f32) {
        for y in 0..image.height {
            for x in 0..image.width {
                let mut pixel = image.pixels[y as usize][x as usize];
                let blue = f32::from(pixel[2]) * factor;
                pixel[2] = blue.min(255.0) as u8;
            }
        }
    }

    pub fn greenify(image: &mut Image, factor: f32) {
        for y in 0..image.height {
            for x in 0..image.width {
                let mut pixel = image.pixels[y as usize][x as usize];
                let green = f32::from(pixel[1]) * factor;
                pixel[1] = green.min(255.0) as u8;
            }
        }
    }

    pub fn warm(image: &mut Image, factor: f32) {
        for y in 0..image.height {
            for x in 0..image.height {
                let mut pixel = image.pixels[y as usize][x as usize];
                let red = f32::from(pixel[0]) * factor;
                let green = f32::from(pixel[1]) * factor;
                pixel[0] = red.min(255.0) as u8;
                pixel[1] = green.min(255.0) as u8;
            }
        }
    }
}
