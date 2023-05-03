use std::{sync::Arc, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use ouou_raytracing::{
    camera::Camera,
    hittable::{
        hittable_list::HittableList,
        instances::{cornell_box, rect::Rectangular, rotate::RotateY, translate::Translate},
        moving_sphere::MovingSphere,
        sphere::Sphere,
    },
    hittablelist,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    renderer,
    textures::image_texture::ImageTexture,
    vec3::{Color, Point3, vec3},
};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion_benchmark");

    group.sample_size(10);
    group.measurement_time(Duration::from_secs(300));
    group.bench_function("cornell_box_bench", |b| {
        b.iter(|| {
            renderer(
                Camera::new(
                    (
                        Point3::new(278.0, 278.0, -800.0),
                        Point3::new(278., 278., 0.),
                    ),
                    vec3(0., 1., 0.),
                    40.,
                    1.,
                    0.0,
                    10.,
                    0.0..1.0,
                ),
                1000,
                3000,
                Color::BLACK,
                &generate_box(),
                "bench",
            )
        })
    });
}

fn generate_box() -> HittableList {
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    hittablelist!(
        Arc::new(cornell_box::cornell_box()),
        Arc::new(Translate::new(
            Arc::new(RotateY::new(
                Arc::new(Rectangular::new(
                    Point3::new(0., 0., 0.),
                    Point3::new(165., 330., 165.),
                    white.clone()
                )),
                15.
            )),
            vec3(265., 0., 295.)
        )),
        Arc::new(Translate::new(
            Arc::new(RotateY::new(
                Arc::new(Rectangular::new(
                    Point3::new(0., 0., 0.),
                    Point3::new(165., 165., 165.),
                    white.clone()
                )),
                -18.
            )),
            vec3(130., 0., 65.)
        )),
        Arc::new(Translate::new(
            Arc::new(Sphere::new(
                Point3::zero(),
                80.,
                Arc::new(Lambertian::with_texture(Arc::new(ImageTexture::from_img(
                    "res/earthmap.jpg"
                ))))
            )),
            vec3(185., 245., 150.)
        )),
        Arc::new(Translate::new(
            Arc::new(MovingSphere::new(
                (Point3::zero(), Point3::new(0., 50., 0.)),
                60.,
                Arc::new(Lambertian::new(Color::new(0.3984375, 0.796875, 0.99))),
                0.0..1.0
            )),
            vec3(350., 180., 40.)
        )),
        Arc::new(Translate::new(
            Arc::new(Sphere::new(
                Point3::zero(),
                60.,
                Arc::new(Dielectric::new(1.5))
            )),
            vec3(350., 60., 40.)
        )),
        Arc::new(Translate::new(
            Arc::new(Sphere::new(
                Point3::zero(),
                60.,
                Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.1))
            )),
            vec3(265. + 165. / 2., 390., 295. + 165. / 2.)
        ))
    )
}
criterion_group!(benches, bench);
criterion_main!(benches);
