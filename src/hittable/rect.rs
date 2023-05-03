use super::*;

/// 与xy坐标轴平行的矩形
pub struct XyRect {
    x: Range<f32>,
    y: Range<f32>,
    z: f32,
    material: Arc<dyn Material>,
}

/// 与xy坐标轴平行的矩形
pub struct XzRect {
    x: Range<f32>,
    y: f32,
    z: Range<f32>,
    material: Arc<dyn Material>,
}

/// 与xy坐标轴平行的矩形
pub struct YzRect {
    x: f32,
    y: Range<f32>,
    z: Range<f32>,
    material: Arc<dyn Material>,
}

impl XyRect {
    pub fn new(x: Range<f32>, y: Range<f32>, z: f32, material: Arc<dyn Material>) -> Self {
        Self { x, y, z, material }
    }
}

impl XzRect {
    pub fn new(x: Range<f32>, y: f32, z: Range<f32>, material: Arc<dyn Material>) -> Self {
        Self { x, y, z, material }
    }
}

impl YzRect {
    pub fn new(x: f32, y: Range<f32>, z: Range<f32>, material: Arc<dyn Material>) -> Self {
        Self { x, y, z, material }
    }
}

impl Hittable for XyRect {
    fn hit(&self, ray_in: &Ray, t_range: &Range<f32>) -> Option<HitRecord> {
        let t = (self.z - ray_in.origin().z()) / ray_in.direction().z;
        if !t_range.contains(&t) {
            None
        } else {
            let x = ray_in.origin().x() + t * ray_in.direction().x;
            let y = ray_in.origin().y() + t * ray_in.direction().y;
            if !self.x.contains(&x) || !self.y.contains(&y) {
                None
            } else {
                let mut rec = HitRecord::new(
                    ray_in.at(t),
                    vec3(0., 0., 1.),
                    t,
                    self.material.as_ref(),
                    (
                        (x - self.x.start) / (self.x.end - self.x.start),
                        (y - self.y.start) / (self.y.end - self.y.start),
                    ),
                );

                rec.set_face_normal(ray_in, rec.normal);
                Some(rec)
            }
        }
    }

    fn bounding_box(&self, _time: &Range<f32>) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x.start, self.y.start, self.z - 0.0001),
            Point3::new(self.x.end, self.y.end, self.z + 0.0001),
        ))
    }
}

impl Hittable for XzRect {
    fn hit(&self, ray_in: &Ray, t_range: &Range<f32>) -> Option<HitRecord> {
        let t = (self.y - ray_in.origin().y()) / ray_in.direction().y;
        if !t_range.contains(&t) {
            None
        } else {
            let x = ray_in.origin().x() + t * ray_in.direction().x;
            let z = ray_in.origin().z() + t * ray_in.direction().z;
            if !self.x.contains(&x) || !self.z.contains(&z) {
                None
            } else {
                let mut rec = HitRecord::new(
                    ray_in.at(t),
                    vec3(0., 1., 0.),
                    t,
                    self.material.as_ref(),
                    (
                        (x - self.x.start) / (self.x.end - self.x.start),
                        (z - self.z.start) / (self.z.end - self.z.start),
                    ),
                );

                rec.set_face_normal(ray_in, rec.normal);
                Some(rec)
            }
        }
    }

    fn bounding_box(&self, _time: &Range<f32>) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x.start, self.z.start, self.y - 0.0001),
            Point3::new(self.x.end, self.z.end, self.y + 0.0001),
        ))
    }
}

impl Hittable for YzRect {
    fn hit(&self, ray_in: &Ray, t_range: &Range<f32>) -> Option<HitRecord> {
        let t = (self.x - ray_in.origin().x()) / ray_in.direction().x;
        if !t_range.contains(&t) {
            None
        } else {
            let y = ray_in.origin().y() + t * ray_in.direction().y;
            let z = ray_in.origin().z() + t * ray_in.direction().z;
            if !self.y.contains(&y) || !self.z.contains(&z) {
                None
            } else {
                let mut rec = HitRecord::new(
                    ray_in.at(t),
                    vec3(1., 0., 0.),
                    t,
                    self.material.as_ref(),
                    (
                        (y - self.y.start) / (self.y.end - self.y.start),
                        (z - self.z.start) / (self.z.end - self.z.start),
                    ),
                );

                rec.set_face_normal(ray_in, rec.normal);
                Some(rec)
            }
        }
    }

    fn bounding_box(&self, _time: &Range<f32>) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.y.start, self.z.start, self.x - 0.0001),
            Point3::new(self.y.end, self.z.end, self.x + 0.0001),
        ))
    }
}
