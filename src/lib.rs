use std::error::Error;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod sphere;
pub mod utility;
pub mod vec3;

pub type MyResult = Result<(), Box<dyn Error>>;
