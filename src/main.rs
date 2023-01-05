use std::{io::{Write, Stdout, stdout, stderr}, error::Error};

use ouou_raytracing::vec3::*;

type MyResult = Result<(), Box<dyn Error>>;

fn write_color(out: &mut Stdout,c: &Color) -> MyResult{
    let o = c.0.to_usize();
    writeln!(out,"{} {} {}", o.0,o.1,o.2)?;
    Ok(())
}
fn main() -> MyResult{
    
    // Image
    const IMAGE_WEIGH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;
    //Render
    println!("P3\n{IMAGE_WEIGH} {IMAGE_HEIGHT}\n255");

    for i in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {i} ");
        for j in 0..IMAGE_WEIGH {
            let col = Color(Vec3::new(i as f64 / (IMAGE_WEIGH - 1) as f64,j as f64 / (IMAGE_HEIGHT - 1) as f64,0.25));
            write_color(&mut stdout(), &col)?;
        }
    }
    eprintln!("\nDone");
    Ok(())
}
