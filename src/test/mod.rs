use crate::{
    camera::Camera,
    color::*,
    hittable::{
        bvh::BvhNode, hittable_list::HittableList, moving_sphere::MovingSphere, sphere::Sphere,
        Hittable,
    },
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    ray::ray_color,
    textures::checker_texture::CheckerTexture,
    vec3::*,
};
use image::ImageBuffer;

use function_name::named;
use rand::{random, thread_rng, Rng};
use rayon::prelude::*;
use std::{ops::Add, sync::Arc, time::SystemTime};

pub fn test_render_random_scene(cam: Camera, world: &dyn Hittable, func_name: &str) {
    // 初始化
    eprintln!("Start Initializing");
    let mut _rng = thread_rng();
    let start_time = SystemTime::now();

    // Image
    // 横纵比
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32; //225
    const SAMPLE_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // Render
    eprintln!("Start Rendering");
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    let mut img_buf = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (i, j, pixel) in img_buf.enumerate_pixels_mut() {
        if j % 30 == 0 && i == 0 {
            eprintln!("\rScanlines remaining: {} ", IMAGE_HEIGHT - j);
        }
        let row = IMAGE_HEIGHT - j;
        let pixel_color = (0..SAMPLE_PER_PIXEL)
            .into_par_iter()
            .map(|_| {
                //将像素坐标转换为场景坐标，然后在附近随机采样
                //gen方法默认就是生成[0,1)的浮点数
                let u = (i as f64 + thread_rng().gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (row as f64 + random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                ray_color(&r, world, MAX_DEPTH)
            })
            .sum();
        *pixel = write_color(&pixel_color, SAMPLE_PER_PIXEL);
    }

    eprintln!("\nFinish Rendering");
    eprintln!("\nDecoding");

    let path = String::from("imgs/test/").add(func_name).add(".png");
    eprintln!("{}", path);
    img_buf.save(&path).expect("保存失败");

    eprintln!("\nDone");
    let finish_time = SystemTime::now();
    eprintln!(
        "time cost: {:.2} seconds",
        finish_time
            .duration_since(start_time)
            .unwrap()
            .as_secs_f64()
    );
}

mod test_bvh;

mod test_checker;

mod test_image_texture;

mod test_perlin;
