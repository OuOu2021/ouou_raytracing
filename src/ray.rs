use crate::vec3::*;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Point3(Vec3::default()),
            dir: Default::default(),
        }
    }
}
impl Ray {
    pub fn new(p: Point3, v: Vec3) -> Self {
        Self { orig: p, dir: v }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        Point3(self.orig.0 + t * self.dir)
    }
}
