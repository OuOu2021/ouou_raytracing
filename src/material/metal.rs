use super::*;
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: clamp(fuzz, 0.0..1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction().to_unit().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_sphere(1.),
            ray_in.time(),
        );
        let attenuation = self.albedo;
        if scattered.direction().dot_mul(rec.normal) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
