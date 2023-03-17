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

pub fn test_render_random_scene(
    cam: Camera,
    sample_per_pixel: u32,
    background: Color,
    world: &dyn Hittable,
    func_name: &str,
) {
    // 初始化
    eprintln!("Start Initializing");
    let mut _rng = thread_rng();
    let start_time = SystemTime::now();

    // Image
    // 横纵比
    const MAX_DEPTH: u32 = 50;
    let image_width = 400.;
    let image_height = 400. / cam.get_aspect_ratio();
    let (image_width, image_height) = (image_width as u32, image_height as u32);
    // Render
    eprintln!("Start Rendering");

    let mut img_buf = ImageBuffer::new(image_width, image_height);
    for (i, j, pixel) in img_buf.enumerate_pixels_mut() {
        if j % 30 == 0 && i == 0 {
            eprintln!("\rScanlines remaining: {} ", image_height - j);
        }
        let row = image_height - j;
        let pixel_color = (0..sample_per_pixel)
            .into_par_iter()
            .map(|_| {
                //将像素坐标转换为场景坐标，然后在附近随机采样
                //gen方法默认就是生成[0,1)的浮点数
                let u = (i as f64 + thread_rng().gen::<f64>()) / (image_width - 1) as f64;
                let v = (row as f64 + random::<f64>()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                ray_color(&r, background, world, MAX_DEPTH)
            })
            .sum();
        *pixel = write_color(&pixel_color, sample_per_pixel);
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

mod test_light;

mod test_cornell_box;
