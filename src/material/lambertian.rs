use std::sync::Arc;

use crate::textures::{solid_color::SolidColor, Texture};

use super::*;
pub struct Lambertian {
    /// 反射率
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(albedo)),
        }
    }
    pub fn with_texture(albedo: Arc<dyn Texture>) -> Self {
        Self {
            albedo: Arc::clone(&albedo),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray, /*漫反射的散射与入射光方向无关，但与时间有关*/
        rec: &HitRecord,
    ) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit(1.);
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((
            Ray::new(rec.p, scatter_direction, ray_in.time()),
            self.albedo.value(rec.texture_uv, rec.p),
        ))
    }
}
