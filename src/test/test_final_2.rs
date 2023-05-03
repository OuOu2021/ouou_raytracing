use crate::{
    hittable::{
        constant_medium::ConstantMedium,
        instances::{rect::Rectangular, rotate::RotateY, translate::Translate},
        rect::XzRect,
    },
    material::diffuse_light::DiffuseLight,
    textures::image_texture::ImageTexture,
};

use super::*;

#[test]
#[named]
fn render_final_2() {
    test_render(
        Camera::new(
            (Point3::new(478., 278., -600.), Point3::new(278., 278., 0.)),
            vec3(0., 1., 0.),
            40.,
            1.,
            0.0,
            10.,
            0.0..1.0,
        ),
        800,
        1000,
        Color::BLACK,
        &final_scene_2(),
        function_name!(),
    );
}

fn final_scene_2() -> HittableList {
    let mut boxes_1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    (0..boxes_per_side).for_each(|i| {
        (0..boxes_per_side).for_each(|j| {
            let i = i as f32;
            let j = j as f32;
            let w = 100.;
            let x_0 = -1000.0 + i * w;
            let z_0 = -1000.0 + j * w;
            let y_0 = 0.;
            let x_1 = x_0 + w;
            let z_1 = z_0 + w;
            let y_1 = thread_rng().gen_range(0.0..96.0);
            boxes_1.add(Arc::new(Rectangular::new(
                Point3::new(x_0, y_0, z_0),
                Point3::new(x_1, y_1, z_1),
                ground.clone(),
            )));
        });
    });

    let mut objs = HittableList::new();
    objs.add(Arc::new(boxes_1));

    let light = Arc::new(DiffuseLight::from_color(Color::new(7., 7., 7.)));
    objs.add(Arc::new(XzRect::new(
        123.0..423.,
        554.0,
        147.0..412.0,
        light,
    )));

    let center_1 = Point3::new(400., 400., 200.);
    let center_2 = center_1 + vec3(30., 0., 0.);
    let moving_sphere_mat = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objs.add(Arc::new(MovingSphere::new(
        (center_1, center_2),
        50.,
        moving_sphere_mat,
        0.0..1.0,
    )));

    objs.add(Arc::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Arc::new(Dielectric::new(1.5)),
    )));

    objs.add(Arc::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        Arc::new(Dielectric::new(1.5)),
    ));
    objs.add(boundary.clone());

    objs.add(Arc::new(ConstantMedium::new_with_color(
        boundary,
        Color::new(0.2, 0.4, 0.9),
        0.2,
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(0., 0., 0.),
        5000.,
        Arc::new(Dielectric::new(1.5)),
    ));
    objs.add(Arc::new(ConstantMedium::new_with_color(
        boundary,
        Color::new(1., 1., 1.),
        0.0001,
    )));

    let earth = Arc::new(Lambertian::with_texture(Arc::new(ImageTexture::from_img(
        "res/earthmap.jpg",
    ))));
    objs.add(Arc::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.,
        earth,
    )));

    let mut boxes_2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    (0..ns).for_each(|_| {
        boxes_2.add(Arc::new(Sphere::new(
            Point3::new(
                thread_rng().gen_range(0.0..165.0),
                thread_rng().gen_range(0.0..165.0),
                thread_rng().gen_range(0.0..165.0),
            ),
            10.,
            white.clone(),
        )));
    });

    objs.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::from_hittable_list(boxes_2, &(0.0..1.0))),
            15.0,
        )),
        vec3(-100., 270., 395.),
    )));

    objs
}
