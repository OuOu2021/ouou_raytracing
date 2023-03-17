use crate::{
    hittable::rect::XyRect, material::diffuse_light::DiffuseLight,
    textures::perlin_texture::NoiseTexture,
};

use super::*;

#[test]
#[named]
fn render_light() {
    test_render_random_scene(
        Camera::new(
            (Point3::new(26., 3.0, 6.), Point3::new(0., 0., 0.)),
            Vec3::new(0., 1., 0.),
            20.,
            16. / 9.,
            0.0,
            10.,
            0.0..1.0,
        ),
        400,
        Color::black(),
        &simple_light(),
        function_name!(),
    );
}

fn simple_light() -> HittableList {
    let mut objs = HittableList::new();
    let pertext = Arc::new(NoiseTexture::default());
    objs.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::with_texture(pertext.clone())),
    )));
    objs.add(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::with_texture(pertext.clone())),
    )));
    let diff_light = Arc::new(DiffuseLight::from_color(Color::new(10., 10., 10.)));
    objs.add(Arc::new(XyRect::new(3.0..5.0, 1.0..3.0, -2., diff_light)));

    objs
}
