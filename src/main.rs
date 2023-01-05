use std::io::{stdin, stdout};

use ouou_raytracing::{color::*, ray::Ray, vec3::*, MyResult};

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin() - center;
    let a = ray.direction().dot_mul(ray.direction());
    let b = 2. * oc.dot_mul(ray.direction());
    let c = oc.dot_mul(oc) - radius * radius;
    let delta = b * b - 4. * a * c ;
    if delta < 0. {
        None
    }
    else {
        //目前两个解都肯定是正的，所以小的那个解就是近处的交点
        Some((-b-delta.sqrt())/(2.*a))
    }
}
/// 接受光线，计算光线打在视口上的颜色
fn ray_color(r: Ray) -> Color {
    //返回交点t值，也就能算出交点坐标
    let t = hit_sphere(Point3::new(0., 0., -1.), 0.5, &r);
    if let Some(tt) = t {
        //计算出朝外的法向量
        let n = r.at(tt) - Point3::new(0.,0.,-1.);
        Color(0.5*Color::new(n.get_x()+1.,n.get_y()+1.,n.get_z()+1.).0)
    } else {
        // 标准化
        let unit_direction = r.direction().to_unit();

        // 把y作为t来线性混合，同时确保t是正数所以做了这么个转换
        let t = 0.5 * (unit_direction.get_y() + 1.0);

        // Blend 公式 天蓝色和白色混合，其实就是二维线性插值
        Color((1. - t) * Color(Vec3::new(1., 1., 1.)).0 + t * Color(Vec3::new(0.5, 0.7, 1.)).0)
    }
}
fn main() -> MyResult {
    // Image
    // 横纵比
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize; //225

    // Camera
    // 视口高度
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    // 焦平面到摄像机的距离
    let focal_length: f64 = 1.0;

    // 原点
    let origin: Point3 = Point3::zero();
    // 水平
    let horizontal: Vec3 = Vec3::new(viewport_width, 0., 0.);
    // 竖直
    let vertical: Vec3 = Vec3::new(0., viewport_height, 0.);
    // 左下角坐标
    let lower_left_corner: Point3 =
        Point3(origin.0 - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length));

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        if j % 10 == 0 {
            eprintln!("\rScanlines remaining: {j} ");
        }
        for i in 0..IMAGE_WIDTH {
            //将像素坐标转换为场景坐标
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner.0 + u * horizontal + v * vertical - origin.0,
            );
            let col = ray_color(r);
            write_color(&mut stdout(), &col)?;
        }
    }
    eprintln!("\nDone");
    Ok(())
}
