use std::sync::Arc;

use super::{solid_color::SolidColor, *};

pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, uv: (f64, f64), p: Point3) -> Color {
        let sines = (10. * p.x()).sin() * (10. * p.y()).sin() * (10. * p.z()).sin();
        if sines < 0. {
            self.odd.value(uv, p)
        } else {
            self.even.value(uv, p)
        }
    }
}

impl CheckerTexture {
    pub fn new(odd: Color, even: Color) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
    pub fn with_texture(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            odd: Arc::clone(&odd),
            even: Arc::clone(&even),
        }
    }
}
