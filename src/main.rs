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
    MyResult,
};
//use rayon::prelude::{IntoParallelIterator, ParallelIterator};
//use rayon::prelude::*;
use std::{
    rc::Rc,
    time::SystemTime,
    {f64::INFINITY, io::stdout},
};

/* 弃用
fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin() - center;
    let a = ray.direction().len_squared();
    let half_b = oc.dot_mul(ray.direction());
    let c = oc.len_squared() - radius * radius;
    let delta = half_b * half_b - a * c;
    if delta < 0. {
        None
    } else {
        //目前两个解都肯定是正的，所以小的那个解就是近处的交点
        Some((-half_b - delta.sqrt()) / a)
    }
}
*/

/// 接受光线，计算光线打在视口上的颜色
fn ray_color(r_in: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::black();
    }
    //返回交点t值，也就能算出交点坐标
    let t = world.hit(&r_in, &(0.001..INFINITY));
    if let Some(rec) = t {
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
    let mut _rng = thread_rng();
    let start_time = SystemTime::now();
    let r: f64 = (PI / 4.).cos();

    // Image
    // 横纵比
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize; //225
    const SAMPLE_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World
    let mut world = HittableList::new();

    let material_left = Rc::new(Lambertian::new(Color::new(0., 0., 1.)));
    let material_right = Rc::new(Lambertian::new(Color::new(1., 0., 0.)));

    world.add(Box::new(Sphere::new(
        Point3::new(-r, 0., -1.),
        r,
        &(material_left as Rc<dyn Material>),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(r, 0., -1.),
        r,
        &(material_right as Rc<dyn Material>),
    )));

    // Camera
    let cam = Camera::new(90., ASPECT_RATIO);

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    (0..IMAGE_HEIGHT).into_iter().rev().for_each(|j| {
        if j % 10 == 0 {
            eprintln!("\rScanlines remaining: {j} ");
        }
        (0..IMAGE_WIDTH).into_iter().for_each(|i| {
            let pixel_color = (0..SAMPLE_PER_PIXEL)
                .into_iter()
                .map(|_| {
                    let mut rng_tmp = thread_rng();
                    //将像素坐标转换为场景坐标，然后在附近随机采样
                    //gen方法默认就是生成[0,1)的浮点数
                    let u = (i as f64 + rng_tmp.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + rng_tmp.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    ray_color(&r, &world, MAX_DEPTH)
                })
                .sum();
            write_color(&mut stdout(), &pixel_color, SAMPLE_PER_PIXEL).expect("写入失败");
        });
    });
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
