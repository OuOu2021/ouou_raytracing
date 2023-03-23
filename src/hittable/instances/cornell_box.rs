use crate::{
    hittable::{
        hittable_list::HittableList,
        rect::{XyRect, XzRect, YzRect},
    },
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    vec3::Color,
};
use std::sync::Arc;

pub fn cornell_box() -> HittableList {
    let mut objs = HittableList::new();
    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15., 15., 15.)));

    objs.add(Arc::new(XyRect::new(
        0.0..555.0,
        0.0..555.0,
        555.0,
        white.clone(),
    )));

    objs.add(Arc::new(YzRect::new(555.0, 0.0..555.0, 0.0..555.0, green)));
    objs.add(Arc::new(YzRect::new(0.0, 0.0..555.0, 0.0..555.0, red)));

    objs.add(Arc::new(XzRect::new(
        0.0..555.0,
        555.0,
        0.0..555.0,
        white.clone(),
    )));
    objs.add(Arc::new(XzRect::new(0.0..555.0, 0.0, 0.0..555.0, white)));

    objs.add(Arc::new(XzRect::new(
        200.0..356.0,
        554.0,
        80.0..332.0,
        light,
    )));

    /*
    // bigger light
    objs.add(Arc::new(XzRect::new(
        150.0..400.0,
        554.0,
        150.0..400.0,
        light,
    )));
    */

    objs
}
