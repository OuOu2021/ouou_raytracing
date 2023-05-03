use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: Vec3) -> Self {
        Self {
            ptr: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(
        &self,
        ray_in: &crate::ray::Ray,
        t_range: &std::ops::Range<f32>,
    ) -> Option<crate::hittable::HitRecord> {
        let ray_moved = Ray::new(
            ray_in.origin() - self.offset,
            ray_in.direction(),
            ray_in.time(),
        );
        if let Some(rec) = self.ptr.hit(&ray_moved, t_range) {
            let mut new_rec = HitRecord::new(
                rec.p + self.offset,
                rec.normal,
                rec.t,
                rec.material,
                rec.texture_uv,
            );
            new_rec.set_face_normal(&ray_moved, new_rec.normal);
            Some(new_rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time: &std::ops::Range<f32>) -> Option<crate::aabb::AABB> {
        self.ptr.bounding_box(time).map(|pre_box| {
            crate::aabb::AABB::new(pre_box.minimum + self.offset, pre_box.maximum + self.offset)
        })
    }
}
