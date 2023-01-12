use std::{ops::Range, rc::Rc};

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    // 交点
    pub p: Point3,
    // 法向量
    pub normal: Vec3,
    pub t: f64,
    pub front_face: Option<bool>,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, material: &Rc<dyn Material>) -> Self {
        Self {
            p,
            normal,
            t,
            front_face: None,
            material: Rc::clone(material),
        }
    }
    pub fn set_face_normal(&mut self, ray_in: &Ray, outward_normal: Vec3) {
        self.front_face = Some(ray_in.direction().dot_mul(outward_normal) < 0.);
        self.normal = if self.front_face == Some(true) {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray_in: &Ray, t_range: &Range<f64>) -> Option<HitRecord>;
}
