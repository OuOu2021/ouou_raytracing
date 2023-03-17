use crate::{
    aabb::AABB,
    hittable::{
        hittable_list::HittableList,
        rect::{XyRect, XzRect, YzRect},
        Hittable,
    },
    material::Material,
    vec3::Point3,
};
use std::sync::Arc;

/// 长方体
pub struct Rectangular {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Rectangular {
    pub fn new(box_min: Point3, box_max: Point3, mat: Arc<dyn Material>) -> Self {
        let mut b: Rectangular = Rectangular {
            box_min,
            box_max,
            sides: HittableList::new(),
        };
        b.sides.add(Arc::new(XyRect::new(
            box_min.x()..box_max.x(),
            box_min.y()..box_max.y(),
            box_max.z(),
            mat.clone(),
        )));
        b.sides.add(Arc::new(XyRect::new(
            box_min.x()..box_max.x(),
            box_min.y()..box_max.y(),
            box_min.z(),
            mat.clone(),
        )));

        b.sides.add(Arc::new(XzRect::new(
            box_min.x()..box_max.x(),
            box_max.y(),
            box_min.z()..box_max.z(),
            mat.clone(),
        )));
        b.sides.add(Arc::new(XzRect::new(
            box_min.x()..box_max.x(),
            box_min.y(),
            box_min.z()..box_max.z(),
            mat.clone(),
        )));

        b.sides.add(Arc::new(YzRect::new(
            box_max.x(),
            box_min.y()..box_max.y(),
            box_min.z()..box_max.z(),
            mat.clone(),
        )));
        b.sides.add(Arc::new(YzRect::new(
            box_min.x(),
            box_min.y()..box_max.y(),
            box_min.z()..box_max.z(),
            mat,
        )));
        b
    }
}

impl Hittable for Rectangular {
    fn hit(
        &self,
        ray_in: &crate::ray::Ray,
        t_range: &std::ops::Range<f64>,
    ) -> Option<crate::hittable::HitRecord> {
        self.sides.hit(ray_in, t_range)
    }

    fn bounding_box(&self, _time: &std::ops::Range<f64>) -> Option<crate::aabb::AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}
