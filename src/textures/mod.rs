use crate::vec3::{Color, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, uv: (f32, f32), p: Point3) -> Color;
}

pub mod checker_texture;
pub mod image_texture;
pub mod perlin_texture;
pub mod solid_color;
