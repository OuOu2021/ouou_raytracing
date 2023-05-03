use crate::{utility::clamp, vec3::*};

pub(crate) fn write_color(pixel_color: &Color, samples_per_pixel: u32) -> image::Rgb<u8> {
    let scale = 1. / samples_per_pixel as f32;
    let (r, g, b) = pixel_color.0.into();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let m = |x: f32| (scale * x).sqrt();
    let (r, g, b) = (m(r), m(g), m(b));

    let rg = 0.0..0.999;
    image::Rgb([
        (256. * clamp(r, rg.clone())) as u8,
        (256. * clamp(g, rg.clone())) as u8,
        (256. * clamp(b, rg)) as u8,
    ])
}
