use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

/*
impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f64 = 16. / 9.;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        // 焦平面到摄像机的距离
        const FOCAL_LENGTH: f64 = 1.0;
        let origin = Point3::zero();
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
        let vertical = Vec3::new(0., VIEWPORT_HEIGHT, 0.);
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: Point3(
                origin.0 - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., FOCAL_LENGTH),
            ),
        }
    }
}
*/

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let random_disk = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * random_disk.get_x() + self.v * random_disk.get_y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner.0 + s * self.horizontal + t * self.vertical
                - self.origin.0
                - offset,
        )
    }

    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        // vertical field-of-view in degrees
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        // distance from focus plane
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).to_unit();
        let u = view_up.cross_mul(w).to_unit();
        let v = w.cross_mul(u);

        //let focal_length = 1.;
        let origin = look_from;
        let horizontal = viewport_width * u * focus_dist;
        let vertical = viewport_height * v * focus_dist;
        let lower_left_corner = Point3(origin.0 - horizontal / 2. - vertical / 2. - w * focus_dist);
        let lens_radius = aperture / 2.;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }
}
