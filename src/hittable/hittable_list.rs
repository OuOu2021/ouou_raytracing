use std::sync::Arc;

use crate::{aabb::surrounding_box, hittable::*, ray::Ray};

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }
    pub fn get_objects(&self) -> &Vec<Arc<dyn Hittable>> {
        &self.objects
    }
    pub fn get_objects_mut(&mut self) -> &mut Vec<Arc<dyn Hittable>> {
        &mut self.objects
    }
}

impl From<HittableList> for Vec<Arc<dyn Hittable>> {
    fn from(value: HittableList) -> Self {
        value.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: &std::ops::Range<f64>) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|x| x.hit(ray, t_range))
            .min_by(|x, y| x.t.partial_cmp(&y.t).expect("无法比较"))
    }

    fn bounding_box(&self, time: &std::ops::Range<f64>) -> Option<crate::aabb::AABB> {
        if self.objects.is_empty() {
            None
        } else if let Some(mut now_box) = self.objects.first().unwrap().bounding_box(time) {
            for i in self.objects.iter().skip(1) {
                if let Some(tmp_box) = i.bounding_box(time) {
                    now_box = surrounding_box(&now_box, &tmp_box);
                } else {
                    return None;
                }
            }
            Some(now_box)
        } else {
            None
        }
    }
}
