use std::{ops::Range, mem::swap};

use rand::{thread_rng, Rng};

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::Hittable,
    hittable_list::HittableList,
};

/// Bounding Volume Hierarchies
/// 层次包围盒
pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Option<Box<dyn Hittable>>,
    bounding_box: AABB,
}

impl BvhNode {
    pub fn from_hittable_list(list: HittableList, t_range: &Range<f64>) -> Self{
        Self::new(list.into(),t_range)
    }
    pub fn new(mut src_object: Vec<Box<dyn Hittable>>, t_range: &Range<f64>) -> Self {
        let comparator = thread_rng().gen_range(0usize..3);
        let mut left: Option<Box<dyn Hittable>>;
        let mut right: Option<Box<dyn Hittable>>;
        match src_object.len() {
            1 => {
                left = src_object.pop();
                right = None;
            }
            2 => {
                let cmp = box_compare(
                    src_object.first().unwrap().as_ref(),
                    src_object.last().unwrap().as_ref(),
                    comparator);
                
                right = src_object.pop();
                left = src_object.pop();
                
                if !cmp{
                    swap(&mut left, &mut right);
                }
            }
            _ => {
                src_object.sort_by(|x, y| {
                    x.bounding_box(t_range)
                        .expect("没有绑定盒无法比较")
                        .minimum
                        .0[comparator]
                        .partial_cmp(
                            &y.bounding_box(t_range)
                                .expect("没有绑定盒无法比较")
                                .minimum
                                .0[comparator],
                        )
                        .expect("浮点数比较出错")
                });
                let mid = src_object.len() / 2;

                let mut tmp_vec = Vec::new();
                for _ in 0..mid {
                    tmp_vec.push(src_object.pop().unwrap());
                }

                left = Some(Box::new(BvhNode::new(src_object, t_range)));
                right = Some(Box::new(BvhNode::new(tmp_vec, t_range)));
            }
        }

        if let Some(r) = right{
            if let (Some(box_l), Some(box_r)) =
                (left.as_ref().unwrap().bounding_box(t_range), r.bounding_box(t_range))
            {
                Self {
                    left: left.unwrap(),
                    right: Some(r),
                    bounding_box: surrounding_box(&box_l, &box_r),
                }
            } else {
                panic!("No bounding box in bvh_node constructor.")
            }
        }
        else {
            let bbox = left.as_ref().unwrap().bounding_box(t_range).unwrap();
            Self {
                left: left.unwrap(),
                right: None,
                bounding_box: bbox,
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        ray_in: &crate::ray::Ray,
        t_range: &std::ops::Range<f64>,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.bounding_box.hit(ray_in, t_range) {
            None
        } else {
            let hit_left = self.left.hit(ray_in, t_range);
            if let Some(left_rec) = hit_left {
                // 会与左边相交：判断与右边能否在更近位置相交而遮挡左边
                if let Some(right) = &self.right{
                    let mut shrunken_t_range = t_range.clone();
                    shrunken_t_range.end = left_rec.t;
                    
                    let hit_right = right.hit(ray_in, &shrunken_t_range);
                    if let Some(right_rec) = hit_right {
                        // 遮挡
                        Some(right_rec)
                    } else {
                        // 不遮挡
                        Some(left_rec)
                    }
                }
                else {
                    Some(left_rec)
                }
                
            } else {
                // 不与左边相交，只考虑右边即可
                if let Some(right) = &self.right{
                    right.hit(ray_in, t_range)
                }
                else {
                    None
                }
            }
        }
    }

    fn bounding_box(&self, _time: &std::ops::Range<f64>) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> bool {
    if let (Some(box_a), Some(box_b)) = (a.bounding_box(&((0.)..0.)), b.bounding_box(&((0.)..0.))) {
        box_a.minimum.0[axis] < box_b.minimum.0[axis]
    } else {
        panic!("没有绑定盒无法比较")
    }
}
