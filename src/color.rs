use crate::{utility::clamp, vec3::*, MyResult};
use std::io::{Stdout, Write};

pub fn write_color(out: &mut Stdout, pixel_color: &Color, samples_per_pixel: u32) -> MyResult {
    let scale = 1. / samples_per_pixel as f64;
    let (r, g, b) = (pixel_color.0 * scale).to_tuple();

    let rg = 0.0..0.999;
    writeln!(
        out,
        "{} {} {}",
        (256. * clamp(r, &rg)) as u8,
        (256. * clamp(g, &rg)) as u8,
        (256. * clamp(b, &rg)) as u8
    )?;
    Ok(())
}
