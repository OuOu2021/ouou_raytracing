use crate::{
    hittable::HitRecord,
    ray::Ray,
    utility::clamp,
    vec3::{Color, Vec3},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    /// 反射率
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray, /*漫反射的散射与入射光无关*/
        rec: &HitRecord,
    ) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit(1.);
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((Ray::new(rec.p, scatter_direction), self.albedo))
    }
}

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
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_sphere(1.));
        let attenuation = self.albedo;
        if scattered.direction().dot_mul(rec.normal) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
