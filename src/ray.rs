use std::f64::INFINITY;

use crate::{hittable::Hittable, vec3::*};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Point3(Vec3::default()),
            dir: Default::default(),
            time: Default::default(),
        }
    }
}
impl Ray {
    pub fn new(p: Point3, v: Vec3, t: f64) -> Self {
        Self {
            orig: p,
            dir: v,
            time: t,
        }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        Point3(self.orig.0 + t * self.dir)
    }
    pub fn time(&self) -> f64 {
        self.time
    }
}

/// 接受光线，计算光线打在视口上的颜色
pub fn ray_color(r_in: &Ray, world: &dyn Hittable, depth: u32) -> Color {
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
