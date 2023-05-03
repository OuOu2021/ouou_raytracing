use std::f32::consts::PI;

use crate::{aabb::AABB, hittable::*, material::Material, vec3::*};
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    /// 返回以原点为球心的单位球上一点的uv坐标
    pub fn get_sphere_uv(p: Point3) -> (f32, f32) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        (phi / (2. * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray_in: &crate::ray::Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord> {
        //光源指向球心
        let oc = ray_in.origin() - self.center;
        let a = ray_in.direction().length_squared();
        let half_b = oc.dot(ray_in.direction());
        let c = oc.length_squared() - self.radius * self.radius;
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
            let uv = Sphere::get_sphere_uv(Point3::new(0., 0., 0.) + outward_normal);
            let mut ans = HitRecord::new(p, outward_normal, t, self.material.as_ref(), uv);
            ans.set_face_normal(ray_in, outward_normal);
            Some(ans)
        }
    }

    fn bounding_box(&self, _time: &std::ops::Range<f32>) -> Option<crate::aabb::AABB> {
        let r = self.radius;
        Some(AABB::new(
            self.center - vec3(r, r, r),
            self.center + vec3(r, r, r),
        ))
    }
}
