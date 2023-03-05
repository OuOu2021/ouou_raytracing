use std::ops::Range;

use crate::{hittable::*, material::Material, vec3::*};
pub struct MovingSphere {
    center: (Point3,Point3),
    radius: f64,
    material: Box<dyn Material + Send + Sync>,
    time: Range<f64>,
}

impl MovingSphere {
    pub fn new(center: (Point3, Point3), radius: f64, material: Box<dyn Material + Send + Sync>, time: Range<f64>) -> Self {
        Self {
            center,
            radius,
            material,
            time,
        }
    }
    pub fn center(&self, time: f64) -> Option<Point3>{
        if self.time.contains(&time){
           Some(self.center.0+((time-self.time.start)/(self.time.end-self.time.start))*(self.center.1-self.center.0))
        }
        else {
            None
        }
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray_in: &crate::ray::Ray, t_range: &std::ops::Range<f64>) -> Option<HitRecord> {
        let center = self.center(ray_in.time()).unwrap();
        //光源指向球心
        let oc = ray_in.origin() - center;
        let a = ray_in.direction().len_squared();
        let half_b = oc.dot_mul(ray_in.direction());
        let c = oc.len_squared() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        if delta < 0. {
            None
        } else {
            let mut root = (-half_b - delta.sqrt()) / a;
            if !t_range.contains(&root) {
                root = (-half_b + delta.sqrt()) / a;
                if !t_range.contains(&root) {
                    return None;
                }
            }
            let t = root;
            let p = ray_in.at(t);
            let outward_normal = (p - center) / self.radius;
            let mut ans = HitRecord::new(p, outward_normal, t, self.material.as_ref());
            ans.set_face_normal(ray_in, outward_normal);
            Some(ans)
        }
    }
}
