use crate::textures::perlin_texture::NoiseTexture;

use super::*;

#[test]
#[named]
fn render_perlin() {
    test_render(
        Default::default(),
        400,
        100,
        Color::new(0.70, 0.80, 1.00),
        &two_perlin_spheres(),
        function_name!(),
    );
}

fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::with_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Arc::new(Lambertian::with_texture(pertext)),
    )));
    world
}
