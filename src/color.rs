use crate::{vec3::*, MyResult};
use std::io::{Stdout, Write};

pub fn write_color(out: &mut Stdout, c: &Color) -> MyResult {
    let o = c.0.to_color();
    writeln!(out, "{} {} {}", o.0, o.1, o.2)?;
    Ok(())
}
