use std::f32::{INFINITY, NEG_INFINITY};

use rand::random;

use crate::{
    textures::{solid_color::SolidColor, Texture},
    vec3::Color,
};

use super::*;

/// 各向同性材质
struct Isotropic {
    albedo: Arc<dyn Texture>,
}

/// 均匀介质
pub struct ConstantMedium {
	// 边界
    boundary: Arc<dyn Hittable>,
	// 相位函数
    phase_function: Arc<dyn Material>,
	// 密度的负倒数
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable>,
        phase_function: Arc<dyn Material>,
        density: f32,
    ) -> Self {
        Self {
            boundary,
            phase_function,
            neg_inv_density: -1. / density,
        }
    }
    pub fn new_with_color(boundary: Arc<dyn Hittable>, col: Color, density: f32) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::from(col)),
            neg_inv_density: -1. / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray_in: &Ray, t_range: &Range<f32>) -> Option<HitRecord> {
        // TODO: Debugging Log
        if let Some(mut rec1) = self.boundary.hit(ray_in, &(NEG_INFINITY..INFINITY)) {
            if let Some(mut rec2) = self.boundary.hit(ray_in, &(rec1.t + 0.0001..INFINITY)) {
                if rec1.t < t_range.start {
                    rec1.t = t_range.start;
                }
                if rec2.t > t_range.end {
                    rec2.t = t_range.end;
                }
                if rec1.t > rec2.t || rec1.t < 0. {
                    None
                } else {
                    let ray_len = ray_in.direction().length();
                    let dist_inside_boundary = (rec2.t - rec1.t) * ray_len;
                    let hit_dist = self.neg_inv_density * random::<f32>().log2();
                    if hit_dist > dist_inside_boundary {
                        None
                    } else {
                        let t = rec1.t + hit_dist / ray_len;
                        Some(HitRecord::new(
                            ray_in.at(t),
                            vec3(1., 0., 0.),
                            t,
                            self.phase_function.as_ref(),
                            (0., 0.),
                        ))
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, time: &Range<f32>) -> Option<AABB> {
        self.boundary.bounding_box(time)
    }
}

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { albedo: tex }
    }
}

impl From<Color> for Isotropic {
    fn from(value: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(value)),
        }
    }
}

impl From<Arc<dyn Texture>> for Isotropic {
    fn from(value: Arc<dyn Texture>) -> Self {
        Isotropic::new(value)
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, crate::vec3::Color)> {
        Some((
            Ray::new(rec.p, Vec3::random_in_sphere(1.), r_in.time()),
            self.albedo.value(rec.texture_uv, rec.p),
        ))
    }
}
