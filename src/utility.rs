pub use rand::*;
pub use std::f64::consts::PI;

pub fn clamp(x: f64, range: &std::ops::Range<f64>) -> f64 {
    if range.contains(&x) {
        x
    } else if range.start > x {
        range.start
    } else {
        range.end
    }
}
