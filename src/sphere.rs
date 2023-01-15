use crate::{hittable::*, material::Material, vec3::*};
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray_in: &crate::ray::Ray, t_range: &std::ops::Range<f64>) -> Option<HitRecord> {
        //光源指向球心
        let oc = ray_in.origin() - self.center;
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
            let outward_normal = (p - self.center) / self.radius;
            let mut ans = HitRecord::new(p, outward_normal, t, self.material.as_ref());
            ans.set_face_normal(&ray_in, outward_normal);
            Some(ans)
        }
    }
}
