use rand::random;

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
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
        let unit_direction = r_in.direction().to_unit();
        let cos_theta = -unit_direction.dot_mul(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random() {
            // 全反射
            Some((
                Ray::new(rec.p, unit_direction.reflect(rec.normal)),
                attenuation,
            ))
        } else {
            // 折射
            Some((
                Ray::new(rec.p, unit_direction.refract(rec.normal, refraction_ratio)),
                attenuation,
            ))
        }
    }
}
