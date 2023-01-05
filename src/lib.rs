use std::error::Error;

pub mod color;
pub mod ray;
pub mod vec3;

pub type MyResult = Result<(), Box<dyn Error>>;
