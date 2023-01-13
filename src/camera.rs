use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

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

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner.0 + u * self.horizontal + v * self.vertical - self.origin.0,
        )
    }

    pub fn new(vfov: f64 /*vertical field-of-view in degrees*/, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;
        let origin = Point3(Vec3::new(0., 0., 0.));
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: Point3(
                origin.0 - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length),
            ),
        }
    }
}
