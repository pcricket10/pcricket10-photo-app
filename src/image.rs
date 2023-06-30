#![allow(dead_code)]
// mod tests;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;

#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Rgb<u8>>>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels: Vec<Vec<Rgb<u8>>> = vec![vec![Rgb([0, 0, 0]); width as usize]; height as usize];
        Image {
            width,
            height,
            pixels,
        }
    }
    pub fn new_from_file(path: &str) -> Result<Self, image::ImageError> {
        let img = image::open(path)?.to_rgb8();
        let width = img.width();
        let height = img.height();
        let mut pixels = vec![vec![Rgb([0, 0, 0]); width as usize]; height as usize];
        for (x, y, pixel) in img.enumerate_pixels() {
            pixels[y as usize][x as usize] = *pixel;
        }
        Ok(Image {
            width,
            height,
            pixels,
        })
    }

    pub fn generate_gradient_image(&mut self, color1: [u8; 3], color2: [u8; 3]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let r: u8 = ((color2[0] as f32 - color1[0] as f32) * (x as f32 / self.width as f32)
                    + color1[0] as f32) as u8;
                let g: u8 = ((color2[1] as f32 - color1[1] as f32)
                    * (y as f32 / self.height as f32)
                    + color1[1] as f32) as u8;
                let b: u8 = ((color2[2] as f32 - color1[2] as f32)
                    * ((x as f32 + y as f32) / (self.width as f32 + self.height as f32))
                    + color1[2] as f32) as u8;

                self.pixels[y as usize][x as usize] = Rgb([r, g, b])
            }
        }
    }

    pub fn to_rgb_image(&self) -> RgbImage {
        let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixels[y as usize][x as usize];
                image.put_pixel(x, y, Rgb([pixel[0], pixel[1], pixel[2]]));
            }
        }
        image
    }
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        let mut resized_pixels =
            vec![vec![Rgb([0, 0, 0]); new_width as usize]; new_height as usize];
        for y in 0..new_height {
            for x in 0..new_width {
                let original_x = (x as f32 / new_width as f32 * self.width as f32) as u32;
                let original_y = (y as f32 / new_height as f32 * self.height as f32) as u32;
                resized_pixels[y as usize][x as usize] =
                    self.pixels[original_y as usize][original_x as usize];
            }
        }
        self.width = new_width;
        self.height = new_height;
        self.pixels = resized_pixels;
    }
    pub fn rotate_90_clockwise(&mut self) {
        let mut rotated_pixels =
            vec![vec![Rgb([0, 0, 0]); self.width as usize]; self.height as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                rotated_pixels[x as usize][(self.height - 1 - y) as usize] =
                    self.pixels[y as usize][x as usize]
            }
        }
        std::mem::swap(&mut self.width, &mut self.height);
        self.pixels = rotated_pixels;
    }
    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) {
        let mut cropped_pixels =
            vec![vec![Rgb([0, 0, 0]); self.width as usize]; self.height as usize];

        for y2 in 0..height {
            for x2 in 0..width {
                cropped_pixels[y2 as usize][x2 as usize] =
                    self.pixels[(y + y2) as usize][(x + x2) as usize];
            }
        }
        self.width = width;
        self.height = height;
        self.pixels = cropped_pixels;
    }
    pub fn add_noise(&mut self, noise_level: u8) {
        let mut rng = rand::thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                let noise_r = rng.gen_range(-(2 * noise_level as i16)..=(2 * noise_level as i16));
                let noise_g = rng.gen_range(-(2 * noise_level as i16)..=(2 * noise_level as i16));
                let noise_b = rng.gen_range(-(2 * noise_level as i16)..=(2 * noise_level as i16));

                let pixel = &mut self.pixels[y as usize][x as usize];
                let pixel_r = pixel[0] as i16 + noise_r;
                let pixel_g = pixel[1] as i16 + noise_g;
                let pixel_b = pixel[2] as i16 + noise_b;

                pixel[0] = pixel_r.clamp(0, 255) as u8;
                pixel[1] = pixel_g.clamp(0, 255) as u8;
                pixel[2] = pixel_b.clamp(0, 255) as u8;
            }
        }
    }
}

pub fn main() {
    // tests::main();
}
