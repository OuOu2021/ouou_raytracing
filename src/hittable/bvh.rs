use rand::{thread_rng, Rng};

use crate::aabb::surrounding_box;

use super::{hittable_list::HittableList, *};

/// Bounding Volume Hierarchies 层次包围盒
/// 
/// 用于加速计算光线与多个物体组合相交。
/// 通过划分空间，将$hit()$方法检索n个物体的复杂度从$O(N)$优化至$O(\log N)$
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: AABB,
}

impl BvhNode {
    pub fn from_hittable_list(mut list: HittableList, t_range: &Range<f64>) -> Self {
        Self::new(list.get_objects_mut(), t_range)
    }
    pub fn new(src_object: &mut [Arc<dyn Hittable>], t_range: &Range<f64>) -> Self {
        let comparator = thread_rng().gen_range(0usize..3);
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        match src_object.len() {
            1 => {
                left = Arc::clone(src_object.first().unwrap());
                right = Arc::clone(src_object.first().unwrap());
            }
            2 => {
                if box_compare(
                    src_object.first().unwrap().as_ref(),
                    src_object.last().unwrap().as_ref(),
                    comparator,
                ) {
                    left = Arc::clone(src_object.first().unwrap());
                    right = Arc::clone(src_object.last().unwrap());
                } else {
                    left = Arc::clone(src_object.last().unwrap());
                    right = Arc::clone(src_object.first().unwrap());
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
                left = Arc::new(BvhNode::new(&mut src_object[..mid], t_range));
                right = Arc::new(BvhNode::new(&mut src_object[mid..], t_range));
            }
        }

        if let (Some(box_l), Some(box_r)) =
            (left.bounding_box(t_range), right.bounding_box(t_range))
        {
            Self {
                left,
                right,
                bounding_box: surrounding_box(&box_l, &box_r),
            }
        } else {
            panic!("No bounding box in bvh_node constructor.")
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
                let mut shrunken_t_range = t_range.clone();
                shrunken_t_range.end = left_rec.t;

                let hit_right = self.right.hit(ray_in, &shrunken_t_range);
                if let Some(right_rec) = hit_right {
                    // 遮挡
                    Some(right_rec)
                } else {
                    // 不遮挡
                    Some(left_rec)
                }
            } else {
                // 不与左边相交，只考虑右边即可
                self.right.hit(ray_in, t_range)
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
