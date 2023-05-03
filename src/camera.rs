use std::ops::Range;

use rand::{thread_rng, Rng};

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3, vec3}, Vec3Funcs,
};

/// 相机类
///
///
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    // shutter and close
    time: Range<f32>,
    aspect_ratio: f32,
}

impl Default for Camera {
    fn default() -> Self {
        let look_from = Point3::new(13., 2.0, 5.);
        let look_at = Point3::new(0., 0., 0.);
        let vup = vec3(0., 1., 0.);
        let dist_to_focus = 10.0;
        let aperture = 0.1;

        Self::new(
            (look_from, look_at),
            vup,
            20.,
            16. / 9.,
            aperture,
            dist_to_focus,
            0.0..1.0,
        )
    }
}

impl Camera {
    ///
    pub fn get_ray(&self, st: (f32, f32)) -> Ray {
        let random_disk = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * random_disk.x + self.v * random_disk.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner.0 + st.0 * self.horizontal + st.1 * self.vertical
                - self.origin.0
                - offset,
            thread_rng().gen_range(self.time.clone()),
        )
    }

    pub fn new(
        // from->at
        look: (Point3, Point3),
        view_up: Vec3,
        // vertical field-of-view in degrees
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        // distance from focus plane
        focus_dist: f32,
        time: Range<f32>,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look.0 - look.1).normalize();
        let u = view_up.cross(w).normalize();
        let v = w.cross(u);

        //let focal_length = 1.;
        let origin = look.0;
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
            _w: w,
            lens_radius,
            time,
            aspect_ratio,
        }
    }
    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}
