use crate::{vec3::*, hittable::*};
pub struct Sphere{
    center: Point3,
    radius: f64
}

impl Sphere{
    pub fn new(center: Point3, radius: f64) -> Self{
        Self{center,radius}
    }
}
impl Hittable for Sphere{
    fn hit(&self, ray: &crate::ray::Ray, t_range: &std::ops::Range<f64>) -> Option<HitRecord> {
        //光源指向球心
        let oc = ray.origin() - self.center;
        let a = ray.direction().len_squared();
        let half_b = oc.dot_mul(ray.direction());
        let c = oc.len_squared() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;
        if delta < 0. {
            None
        } else {
            let mut root = (-half_b - delta.sqrt())/a;
            if !t_range.contains(&root) {
                root = (-half_b + delta.sqrt())/a;
                if !t_range.contains(&root){
                    return None;
                }
            }
            let t = root;
            let p = ray.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let mut ans = HitRecord::new(p, outward_normal, t);
            ans.set_face_normal(&ray, outward_normal);
            Some(ans)
        }
    }
}