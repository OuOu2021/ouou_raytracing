use std::sync::Arc;

use crate::textures::{solid_color::SolidColor, Texture};

use super::*;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_color(col: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(col)),
        }
    }

    pub fn from_texture(text: Arc<dyn Texture>) -> Self {
        Self { emit: text }
    }
}
impl Default for DiffuseLight {
    fn default() -> Self {
        Self {
            emit: Arc::new(SolidColor::new(Color::white())),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, uv: (f64, f64), p: Point3) -> Color {
        self.emit.value(uv, p)
    }
}
