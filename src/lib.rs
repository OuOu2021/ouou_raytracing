use std::error::Error;

use camera::Camera;
use hittable::Hittable;
use image::ImageBuffer;
use rand::random;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use vec3::Color;

use crate::{ray::ray_color, color::write_color};

pub mod aabb;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod textures;
pub mod utility;
pub mod vec3;

#[cfg(test)]
mod test;

pub type MyResult = Result<(), Box<dyn Error>>;

pub fn renderer(
    cam: Camera,
    image_width: u32,
    sample_per_pixel: u32,
    background: Color,
    world: &dyn Hittable,
    output_name: &str,
) {
    // 初始化
    eprintln!("Start Initializing");
    // Image
    // 横纵比
    const MAX_DEPTH: u32 = 50;
    let image_width = image_width as f64;
    let image_height = image_width / cam.get_aspect_ratio();
    let (image_width, image_height) = (image_width as u32, image_height as u32);
    // Render
    eprintln!("Start Rendering");

    let mut img_buf = ImageBuffer::new(image_width, image_height);
    for (i, j, pixel) in img_buf.enumerate_pixels_mut() {
        if j % 30 == 0 && i == 0 {
            eprint!("\rScanlines remaining: {} ", image_height - j);
        }
        let row = image_height - j;
        let pixel_color = (0..sample_per_pixel)
            .into_par_iter()
            .map(|_| {
                // 将像素坐标转换为场景坐标，然后在附近随机采样
                // gen方法默认就是生成[0,1)的浮点数
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (row as f64 + random::<f64>()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                ray_color(&r, background, world, MAX_DEPTH)
            })
            .sum();
        *pixel = write_color(&pixel_color, sample_per_pixel);
    }

    eprintln!("\nFinish Rendering");
    eprintln!("\nDecoding");

    let path = String::from(output_name)+".png";
    eprintln!("{}", path);
    img_buf.save(&path).expect("保存失败");

    eprintln!("\nDone");
}