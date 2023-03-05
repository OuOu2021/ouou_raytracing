use image::ImageBuffer;
use ouou_raytracing::{
    camera::Camera,
    color::*,
    hittable::*,
    hittable_list::*,
    material::{Dielectric, Lambertian, Material, Metal},
    ray::Ray,
    sphere::Sphere,
    utility::*,
    vec3::*,
    MyResult, moving_sphere::MovingSphere,
};

use rayon::prelude::*;
use std::{f64::INFINITY, time::SystemTime};

/// 接受光线，计算光线打在视口上的颜色
fn ray_color(r_in: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    // 超过反射次数限制，返回黑色
    if depth == 0 {
        return Color::black();
    }

    // 左边界0.001而非0是为了避免误差导致射中物体内部
    if let Some(rec) = world.hit(r_in, &(0.001..INFINITY)) {
        if let Some((scattered, attenuation)) = rec.material.scatter(r_in, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::black()
        }
    } else {
        // 标准化
        let unit_direction = r_in.direction().to_unit();

        // 把y作为t来线性混合，同时确保t是正数所以做了这么个转换
        let t = 0.5 * (unit_direction.get_y() + 1.0);

        // Blend 公式 天蓝色和白色混合，其实就是二维线性插值
        (1. - t) * Color(Vec3::new(1., 1., 1.)) + t * Color(Vec3::new(0.5, 0.7, 1.))
    }
}

fn main() -> MyResult {
    // 初始化
    eprintln!("Start Initializing");
    let mut _rng = thread_rng();
    let start_time = SystemTime::now();
    
    // Image
    // 横纵比
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32; //225
    const SAMPLE_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 500;

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13., 2.0, 5.);
    let look_at = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0..1.0
    );

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
                ray_color(&r, &world, MAX_DEPTH)
            })
            .sum();
        *pixel = write_color(&pixel_color, SAMPLE_PER_PIXEL);
    }

    eprintln!("\nFinish Rendering");
    eprintln!("\nDecoding");

    let path = "./products/test.png";
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
    Ok(())
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));
    let c = [
        Point3::new(0., 1., 0.),
        Point3::new(-4.0, 1., 0.),
        Point3::new(4.0, 1., 0.),
    ];
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let mut center = Point3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );
            loop {
                if (center - c[0]).len() < 1.2
                    || (center - c[1]).len() < 1.2
                    || (center - c[2]).len() < 1.2
                {
                    center = Point3::new(
                        a as f64 + 0.9 * random::<f64>(),
                        0.2,
                        b as f64 + 0.9 * random::<f64>(),
                    );
                } else {
                    break;
                }
            }

            if (center - Point3::new(4., 0.2, 0.)).len() > 0.9 {
                let sphere_material: Box<dyn Material + Send + Sync>;
                match choose_mat {
                    a if (0.0..=0.8).contains(&a) => {
                        // diffuse
                        let albedo = Color(Vec3::random(0.0..1.)) * Color(Vec3::random(0.0..1.));
                        sphere_material = Box::new(Lambertian::new(albedo));
                        
                        let center_2 = center +Vec3::new(0., thread_rng().gen_range(0.0..0.5), 0.);
                        world.add(Box::new(MovingSphere::new((center,center_2), 0.2, sphere_material, 0.0..1.0)));
                    }
                    b if (0.0..=0.95).contains(&b) => {
                        // metal
                        let albedo = Color(Vec3::random(0.0..1.)) * Color(Vec3::random(0.0..1.));
                        let fuzz = thread_rng().gen_range(0.0..0.5);

                        sphere_material = Box::new(Metal::new(albedo, fuzz));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    _ => {
                        // glass

                        sphere_material = Box::new(Dielectric::new(1.5));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    }

    let material_1 = Box::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(c[0], 1.0, material_1)));

    let material_2 = Box::new(Lambertian::new(Color::new(0.3984375, 0.796875, 0.99)));
    world.add(Box::new(Sphere::new(c[1], 1.0, material_2)));

    let material_3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(c[2], 1.0, material_3)));

    world
}

fn _scene_2() -> HittableList {
    let mut world = HittableList::new();
    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(Dielectric::new(1.5));
    let material_left_1 = Box::new(Dielectric::new(1.5));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.05));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left_1,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.3,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    world
}
