use crate::{hittable::*, ray::Ray};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
unsafe impl Sync for HittableList {}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: &std::ops::Range<f64>) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|x| x.hit(&ray, &t_range))
            .min_by(|x, y| x.t.partial_cmp(&y.t).expect("无法比较"))
    }
}
