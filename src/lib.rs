use std::error::Error;

pub mod color;
pub mod ray;
pub mod vec3;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
mod utility;

pub type MyResult = Result<(), Box<dyn Error>>;
