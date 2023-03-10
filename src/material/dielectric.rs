use super::*;

pub struct Dielectric {
    /// Index of Refraction,折射率
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        // Use(Copy) Schlick's approximation for reflectance.
        let r0 = ((1. - ref_index) / (1. + ref_index)).powf(2.);
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(0.98, 0.98, 0.98);
        let refraction_ratio = if let Some(a) = rec.front_face {
            if a {
                1.0 / self.ir
            } else {
                self.ir
            }
        } else {
            panic!()
        };
        let unit_direction = ray_in.direction().to_unit();
        let cos_theta = -unit_direction.dot_mul(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random() {
            // 全反射
            Some((
                Ray::new(rec.p, unit_direction.reflect(rec.normal), ray_in.time()),
                attenuation,
            ))
        } else {
            // 折射
            Some((
                Ray::new(
                    rec.p,
                    unit_direction.refract(rec.normal, refraction_ratio),
                    ray_in.time(),
                ),
                attenuation,
            ))
        }
    }
}
