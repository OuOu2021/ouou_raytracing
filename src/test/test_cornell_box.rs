use crate::hittable::instances::{cornell_box, rect::Rectangular};

use super::*;

#[test]
#[named]
fn render_cornell_box() {
    test_render_random_scene(
        Camera::new(
            (
                Point3::new(278.0, 278.0, -800.0),
                Point3::new(278., 278., 0.),
            ),
            Vec3::new(0., 1., 0.),
            40.,
            1.,
            0.0,
            10.,
            0.0..1.0,
        ),
        200,
        Color::black(),
        &generate_box(),
        function_name!(),
    );
}

fn generate_box() -> HittableList {
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let mut hl = HittableList::new();
    hl.add(Arc::new(cornell_box::cornell_box()));

    hl.add(Arc::new(Rectangular::new(
        Point3::new(130., 0., 65.),
        Point3::new(295., 165., 230.),
        white.clone(),
    )));
    hl.add(Arc::new(Rectangular::new(
        Point3::new(265., 0., 295.),
        Point3::new(430., 330., 460.),
        white.clone(),
    )));

    hl
}
