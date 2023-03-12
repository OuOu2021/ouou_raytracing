use std::error::Error;

pub mod aabb;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod textures;
pub mod utility;
pub mod vec3;

#[cfg(test)]
mod test;

pub type MyResult = Result<(), Box<dyn Error>>;
