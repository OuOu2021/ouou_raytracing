use rand::random;
use crate::{
    hittable::HitRecord,
    ray::Ray,
    utility::clamp,
    vec3::{Color, Point3, Vec3, Vec3Funcs},
};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
    fn emitted(&self, _uv: (f32, f32), _p: Point3) -> Color {
        // 默认不发光
        Color::BLACK
    }
}

pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;
