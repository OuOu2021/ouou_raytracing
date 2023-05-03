use crate::aabb::surrounding_box;

use super::{sphere::Sphere, *};

/// 时间范围内匀速运动的球体(超出时间范围仍在运动)
pub struct MovingSphere {
	// 运动的起止位置
    center: (Point3, Point3),
    radius: f32,
    material: Arc<dyn Material>,
	// 运动的时间范围
    time: Range<f32>,
}

impl MovingSphere {
    pub fn new(
        center: (Point3, Point3),
        radius: f32,
        material: Arc<dyn Material>,
        time: Range<f32>,
    ) -> Self {
        Self {
            center,
            radius,
            material,
            time,
        }
    }
    pub fn center(&self, time: f32) -> Point3 {
        self.center.0
            + ((time - self.time.start) / (self.time.end - self.time.start))
                * (self.center.1 - self.center.0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray_in: &crate::ray::Ray, t_range: &std::ops::Range<f32>) -> Option<HitRecord> {
        let center = self.center(ray_in.time());
        //光源指向球心
        let oc = ray_in.origin() - center;
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
            let outward_normal = (p - center) / self.radius;
            let uv = Sphere::get_sphere_uv(Point3::new(0., 0., 0.) + outward_normal);
            let mut ans = HitRecord::new(p, outward_normal, t, self.material.as_ref(), uv);
            ans.set_face_normal(ray_in, outward_normal);
            Some(ans)
        }
    }

    fn bounding_box(&self, time: &Range<f32>) -> Option<crate::aabb::AABB> {
        let r = self.radius;
        let box0 = AABB::new(
            self.center(time.start) - vec3(r, r, r),
            self.center(time.start) + vec3(r, r, r),
        );
        let box1 = AABB::new(
            self.center(time.end) - vec3(r, r, r),
            self.center(time.end) + vec3(r, r, r),
        );
        Some(surrounding_box(&box0, &box1))
    }
}
