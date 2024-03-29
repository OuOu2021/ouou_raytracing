use std::f32::INFINITY;

use crate::{hittable::Hittable, vec3::*};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f32,
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
    pub fn new(p: Point3, v: Vec3, t: f32) -> Self {
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
    pub fn at(&self, t: f32) -> Point3 {
        Point3(self.orig.0 + t * self.dir)
    }
    pub fn time(&self) -> f32 {
        self.time
    }
}

/// 接受光线，计算光线打在视口上的颜色
pub fn ray_color(r_in: &Ray, background: Color, world: &dyn Hittable, depth: u32) -> Color {
    // 超过反射次数限制，返回黑色
    if depth == 0 {
        return Color::BLACK;
    }

    // 左边界0.001而非0是为了避免误差导致射中物体内部
    if let Some(rec) = world.hit(r_in, &(0.001..INFINITY)) {
        let emitted = rec.material.emitted(rec.texture_uv, rec.p);
        if let Some((scattered, attenuation)) = rec.material.scatter(r_in, &rec) {
            emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        /*
        // 标准化
        let unit_direction = r_in.direction().to_unit();

        // 把y作为t来线性混合，同时确保t是正数所以做了这么个转换
        let t = 0.5 * (unit_direction.y() + 1.0);

        // Blend 公式 天蓝色和白色混合，其实就是二维线性插值
        (1. - t) * Color(vec3(1., 1., 1.)) + t * Color(vec3(0.5, 0.7, 1.))
        */
        background
    }
}
