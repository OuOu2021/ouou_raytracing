use std::mem::swap;

use crate::vec3::Point3;

/// Axis-Aligned Bounding Boxes
/// 坐标轴对齐的包围盒
#[derive(Copy, Clone)]
pub struct AABB {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self {
            minimum: a,
            maximum: b,
        }
    }
    pub fn hit(&self, ray_in: &crate::ray::Ray, t_range: &std::ops::Range<f64>) -> bool {
        let mut r = t_range.clone();

        for i in 0..3 {
            let inv_d = 1. / ray_in.direction()[i];
            let mut t0 = (self.minimum[i] - ray_in.origin()[i]) * inv_d;
            let mut t1 = (self.maximum[i] - ray_in.origin()[i]) * inv_d;

            if inv_d < 0. {
                swap(&mut t0, &mut t1);
            }
            r.start = t_range.start.max(t0);
            r.end = t_range.end.min(t1);
            if r.end <= r.start {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(a: &AABB, b: &AABB) -> AABB {
    let small = Point3::new(
        a.minimum.0[0].min(b.minimum.0[0]),
        a.minimum.0[1].min(b.minimum.0[1]),
        a.minimum.0[2].min(b.minimum.0[2]),
    );
    let big = Point3::new(
        a.maximum.0[0].max(b.maximum.0[0]),
        a.maximum.0[1].max(b.maximum.0[1]),
        a.maximum.0[2].max(b.maximum.0[2]),
    );
    AABB::new(small, big)
}
