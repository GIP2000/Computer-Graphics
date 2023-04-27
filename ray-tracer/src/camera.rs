use cgmath::{vec3, Angle, InnerSpace, Point3, Rad, Vector3};

use crate::{ray::Ray, vector_additon::VectorAdditions};

pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3<f64>,
        lookat: Point3<f64>,
        vup: Vector3<f64>,
        theta: Rad<f64>,
        aspect_ratio: f64,
        apperature: f64,
        focus_dist: f64,
    ) -> Self {
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = lookfrom - horizontal / 2. - vertical / 2. - focus_dist * w;
        let lens_radius = apperature / 2.;

        return Self {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        };
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vector3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        );
    }
}
