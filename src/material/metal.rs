use super::*;
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: clamp(fuzz, 0.0..1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction().normalize().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_sphere(1.),
            ray_in.time(),
        );
        let attenuation = self.albedo;
        if scattered.direction().dot(rec.normal) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
