use super::*;

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color_value: color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: (f64, f64), _p: crate::vec3::Point3) -> Color {
        self.color_value
    }
}
