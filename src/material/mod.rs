use rand::random;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    utility::clamp,
    vec3::{Color, Vec3},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub mod dielectric;
pub mod lambertian;
pub mod metal;
