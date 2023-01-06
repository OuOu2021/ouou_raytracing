use crate::{vec3::{Vec3,Color,Point3},ray::Ray,hittable::*};

pub struct HittableList{
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList{
    pub fn new() -> Self{
        Self{objects: Vec::new()}
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self,obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList{
    fn hit(&self, ray: &Ray, t_range: &std::ops::Range<f64>) -> Option<HitRecord>{
        let mut ans = self.objects.iter().filter_map(|x|{
            x.hit(&ray, &t_range)
        }).collect::<Vec<_>>();
        ans.sort_by(|x,y|{
            x.t.partial_cmp(&y.t).expect("无法比较")
        });
        ans.into_iter().next()
    }
}