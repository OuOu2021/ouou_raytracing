pub use rand::*;
pub use std::f32::consts::PI;

pub fn clamp(x: f32, range: std::ops::Range<f32>) -> f32 {
    if range.contains(&x) {
        x
    } else if range.start > x {
        range.start
    } else {
        range.end
    }
}
