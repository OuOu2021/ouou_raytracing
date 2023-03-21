use std::{sync::Arc, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use ouou_raytracing::{renderer, camera::Camera, vec3::{Color, Vec3, Point3}, hittable::{hittable_list::HittableList, instances::{cornell_box, rect::Rectangular}}, material::lambertian::Lambertian, hittablelist};

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("criterion_benchmark");

    group.sample_size(10);
    group.measurement_time(Duration::from_secs(300));
    group.bench_function("cornell_box_bench", |b| b.iter(|| renderer(Camera::new(
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
    400,
    50,
    Color::black(),
    &generate_box(),
    "bench",)));
}

fn generate_box() -> HittableList {
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    hittablelist!(
        Arc::new(cornell_box::cornell_box()),
        Arc::new(Rectangular::new(
            Point3::new(130., 0., 65.),
            Point3::new(295., 165., 230.),
            white.clone(),
        )),
        Arc::new(Rectangular::new(
            Point3::new(265., 0., 295.),
            Point3::new(430., 330., 460.),
            white.clone(),
        ))
    )
}
criterion_group!(benches, bench);
criterion_main!(benches);