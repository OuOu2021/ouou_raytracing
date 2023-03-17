use rand::random;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    utility::clamp,
    vec3::{Color, Point3, Vec3},
};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
    fn emitted(&self, _uv: (f64, f64), _p: Point3) -> Color {
        // 默认不发光
        Color::black()
    }
}

pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;
