use crate::{
    aabb::AABB,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3, Vec3Funcs, vec3},
};
use std::{ops::Range, sync::Arc};

pub struct HitRecord<'a> {
    // 交点
    pub p: Point3,
    // 法向量
    pub normal: Vec3,
    pub t: f32,
    pub front_face: Option<bool>,
    pub material: &'a dyn Material,

    // uv coordinates of hit point's texture
    pub texture_uv: (f32, f32),
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f32,
        material: &'a dyn Material,
        uv: (f32, f32),
    ) -> Self {
        Self {
            p,
            normal,
            t,
            front_face: None,
            material,
            texture_uv: uv,
        }
    }
    pub fn set_face_normal(&mut self, ray_in: &Ray, outward_normal: Vec3) {
        self.front_face = Some(ray_in.direction().dot(outward_normal) < 0.);
        self.normal = if self.front_face == Some(true) {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray_in: &Ray, t_range: &Range<f32>) -> Option<HitRecord>;
    fn bounding_box(&self, time: &Range<f32>) -> Option<AABB>;
}

pub mod bvh;
pub mod constant_medium;
pub mod hittable_list;
pub mod instances;
pub mod moving_sphere;
pub mod rect;
pub mod sphere;
