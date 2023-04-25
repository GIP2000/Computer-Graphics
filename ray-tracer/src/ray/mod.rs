pub mod hittable;

use std::f64::INFINITY;

use cgmath::{vec3, ElementWise, EuclideanSpace, InnerSpace, Point3, Vector3};

use self::hittable::Hittable;

pub struct Ray {
    orig: Point3<f64>,
    dir: Vector3<f64>,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            orig: Point3::new(0., 0., 0.),
            dir: vec3(0., 0., 0.),
        }
    }
}

impl Ray {
    pub fn new(orig: Point3<f64>, dir: Vector3<f64>) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        return self.orig + (t * self.dir);
    }

    pub fn color(&self, world: &dyn Hittable) -> Vector3<f64> {
        if let Some(rec) = world.hit(self, 0., INFINITY) {
            return 0.5 * (rec.normal + vec3(1., 1., 1.));
        }
        let unit_direction = self.dir.normalize();
        let t = 0.5 * (unit_direction.y + 1.);
        return (1. - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
    }
}
