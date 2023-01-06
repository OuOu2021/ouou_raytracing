use std::ops::Range;

use crate::{vec3::{Vec3,Color,Point3},ray::Ray};

#[derive(Debug)]
pub struct HitRecord{
    // 交点
    pub p: Point3,
    // 法向量
    pub normal: Vec3,
    pub t: f64,
    pub front_face : Option<bool>
}

impl HitRecord{
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self{Self{p,normal,t,front_face:None}}
    pub fn set_face_normal(&mut self,r: &Ray, outward_normal: Vec3){
        self.front_face = Some(r.direction().dot_mul(outward_normal) < 0.);
        self.normal = if self.front_face == Some(true) {outward_normal} else {-outward_normal};
    }
}

pub trait Hittable{
    fn hit(&self, r: &Ray, t_range: &Range<f64>) -> Option<HitRecord>;
}