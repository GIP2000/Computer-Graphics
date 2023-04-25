use cgmath::{vec3, Point3, Vector3};

use crate::ray::Ray;

pub struct Camera {
    origin: Point3<f64>,
    lower_left_corner: Point3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
}

impl Camera {
    pub fn new(
        origin: Point3<f64>,
        aspect_ratio: f64,
        viewport_height: f64,
        focal_length: f64,
    ) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = vec3(viewport_width, 0., 0.);
        let vertical = vec3(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - vec3(0., 0., focal_length);
        return Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        };
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        );
    }
}
