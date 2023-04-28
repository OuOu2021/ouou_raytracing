use crate::hittable::{
    constant_medium::ConstantMedium,
    instances::{
        cornell_box::cornell_box, rect::Rectangular, rotate::RotateY, translate::Translate,
    },
};

use super::*;

#[test]
#[named]
fn render_medium() {
    test_render(
        Camera::new(
            (Point3::new(278., 278.0, -800.), Point3::new(278., 278., 0.)),
            Vec3::new(0., 1., 0.),
            40.,
            1.,
            0.0,
            10.,
            0.0..1.0,
        ),
        800,
        400,
        Color::black(),
        &cornell_smoke(),
        function_name!(),
    );
}

fn cornell_smoke() -> HittableList {
    let mut objs = HittableList::new();

    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    objs.add(Arc::new(cornell_box()));

    let box1 = Arc::new(Rectangular::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    ));
    let box1 = Arc::new(RotateY::new(box1, 15.));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
    let box1 = Arc::new(ConstantMedium::new_with_color(
        box1,
        Color::new(0., 0., 0.),
        0.01,
    ));
    objs.add(box1);

    let box2 = Arc::new(Rectangular::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white.clone(),
    ));
    let box2 = Arc::new(RotateY::new(box2, -18.));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    let box2 = Arc::new(ConstantMedium::new_with_color(
        box2,
        Color::new(1., 1., 1.),
        0.01,
    ));
    objs.add(box2);

    objs
}
