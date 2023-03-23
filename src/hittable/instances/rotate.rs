use std::{
    f64::{INFINITY, NEG_INFINITY},
    sync::Arc,
};

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> Self {
        let rad = angle.to_radians();
        let (sin, cos) = (rad.sin(), rad.cos());
        if let Some(bbox) = p.bounding_box(&(0.0..1.0)) {
            let (mut min, mut max) = (
                Point3::new(INFINITY, INFINITY, INFINITY),
                Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
            );

            (0..2).for_each(|i| {
                (0..2).for_each(|j| {
                    (0..2).for_each(|k| {
                        let (i, j, k) = (i as f64, j as f64, k as f64);
                        let (x, y, z) = (
                            i * bbox.maximum.x() + (1. - i) * bbox.minimum.x(),
                            j * bbox.maximum.y() + (1. - j) * bbox.minimum.y(),
                            k * bbox.maximum.z() + (1. - k) * bbox.minimum.z(),
                        );
                        let (newx, newz) = (cos * x + sin * z, -sin * x + cos * z);
                        let tester = Vec3::new(newx, y, newz);
                        (0..3).for_each(|c| {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        });
                    });
                });
            });
            Self {
                ptr: p,
                sin_theta: rad.sin(),
                cos_theta: rad.cos(),
                bbox: Some(AABB::new(min, max)),
            }
        } else {
            Self {
                ptr: p,
                sin_theta: rad.sin(),
                cos_theta: rad.cos(),
                bbox: None,
            }
        }
    }
}

impl Hittable for RotateY {
    fn hit(
        &self,
        ray_in: &crate::ray::Ray,
        t_range: &std::ops::Range<f64>,
    ) -> Option<crate::hittable::HitRecord> {
        let mut origin = ray_in.origin();
        let mut direction = ray_in.direction();

        origin[0] = self.cos_theta * ray_in.origin()[0] - self.sin_theta * ray_in.origin()[2];
        origin[2] = self.sin_theta * ray_in.origin()[0] + self.cos_theta * ray_in.origin()[2];

        direction[0] =
            self.cos_theta * ray_in.direction()[0] - self.sin_theta * ray_in.direction()[2];
        direction[2] =
            self.sin_theta * ray_in.direction()[0] + self.cos_theta * ray_in.direction()[2];

        let ray_rotate = Ray::new(origin, direction, ray_in.time());
        if let Some(rec) = self.ptr.hit(&ray_rotate, t_range) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            let mut rec_rotate = HitRecord::new(p, normal, rec.t, rec.material, rec.texture_uv);
            rec_rotate.set_face_normal(&ray_rotate, normal);
            Some(rec_rotate)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time: &std::ops::Range<f64>) -> Option<AABB> {
        self.bbox
    }
}
