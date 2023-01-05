use std::io::{stdout, stdin};

use ouou_raytracing::{color::*, vec3::*, MyResult, ray::Ray};

/// 接受光线，计算光线打在视口上的颜色
fn ray_color(r: Ray) -> Color{
    // 标准化
    let unit_direction = r.direction().to_unit();

    let t = 0.5*(unit_direction.get_y() + 1.0);

    // Blend 公式 天蓝色和白色混合
    Color((1. - t)*Color(Vec3::new(1.,1.,1.)).0+t*Color(Vec3::new(0.5,0.7,1.)).0)
}
fn main() -> MyResult {
    // Image
    const ASPECT_RATIO: f64 = 16./9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    
    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point3 = Point3::zero(); 
    let horizontal: Vec3 = Vec3::new(viewport_width,0.,0.);
    let vertical: Vec3 = Vec3::new(0.,viewport_height,0.);
    let lower_left_corner: Point3 = Point3(origin.0 - horizontal/2. - vertical/2. - Vec3::new(0.,0.,focal_length));
    
    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {j} ");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(origin,lower_left_corner.0 + u * horizontal + v * vertical - origin.0);
            let col = ray_color(r);
            write_color(&mut stdout(), &col)?;
        }
    }
    eprintln!("\nDone");
    Ok(())
}
