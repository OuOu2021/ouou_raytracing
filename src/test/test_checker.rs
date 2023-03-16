use super::*;
use crate::test::test_render_random_scene;

#[test]
#[named]
fn render_scene_with_checker() {
    test_render_random_scene(
        Default::default(),
        &random_scene_with_checker(),
        &function_name!(),
    );
}

fn random_scene_with_checker() -> BvhNode {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::with_texture(checker));

    world.add(Arc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));
    let c = [
        Point3::new(0., 1., 0.),
        Point3::new(-4.0, 1., 0.),
        Point3::new(4.0, 1., 0.),
    ];
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let mut center = Point3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );
            loop {
                if (center - c[0]).len() < 1.2
                    || (center - c[1]).len() < 1.2
                    || (center - c[2]).len() < 1.2
                {
                    center = Point3::new(
                        a as f64 + 0.9 * random::<f64>(),
                        0.2,
                        b as f64 + 0.9 * random::<f64>(),
                    );
                } else {
                    break;
                }
            }

            if (center - Point3::new(4., 0.2, 0.)).len() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync>;
                match choose_mat {
                    a if (0.0..=0.8).contains(&a) => {
                        // diffuse
                        let albedo = Color(Vec3::random(0.0..1.)) * Color(Vec3::random(0.0..1.));
                        sphere_material = Arc::new(Lambertian::new(albedo));

                        let center_2 = center + Vec3::new(0., thread_rng().gen_range(0.0..0.5), 0.);
                        world.add(Arc::new(MovingSphere::new(
                            (center, center_2),
                            0.2,
                            sphere_material,
                            0.0..1.0,
                        )));
                    }
                    b if (0.0..=0.95).contains(&b) => {
                        // metal
                        let albedo = Color(Vec3::random(0.0..1.)) * Color(Vec3::random(0.0..1.));
                        let fuzz = thread_rng().gen_range(0.0..0.5);

                        sphere_material = Arc::new(Metal::new(albedo, fuzz));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    _ => {
                        // glass
                        sphere_material = Arc::new(Dielectric::new(1.5));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(c[0], 1.0, material_1)));

    let m_checker = CheckerTexture::new(
        Color::new(0.3984375, 0.796875, 0.99),
        Color::new(0.9, 0.9, 0.9),
    );
    let material_2 = Arc::new(Lambertian::with_texture(Arc::new(m_checker)));
    world.add(Arc::new(Sphere::new(c[1], 1.0, material_2)));

    let material_3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(c[2], 1.0, material_3)));

    BvhNode::from_hittable_list(world, &(0.0..1.0))
}
