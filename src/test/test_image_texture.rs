use super::*;
use crate::textures::image_texture::ImageTexture;

#[test]
#[named]
fn render_earth() {
    test_render_random_scene(cam(), &earth(), &function_name!());
}

fn cam() -> Camera {
    const ASPECT_RATIO: f64 = 16. / 9.;
    let look_from = Point3::new(13., 2.0, 3.);
    let look_at = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    Camera::new(
        (look_from, look_at),
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0..1.0,
    )
}

fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::from_img("res/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::with_texture(earth_texture));
    let globe = Sphere::new(Point3::zero(), 2., earth_surface);
    let mut list = HittableList::new();
    list.add(Arc::new(globe));
    list
}
