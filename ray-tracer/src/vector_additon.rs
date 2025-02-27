use std::ops::Range;

use cgmath::{vec3, InnerSpace, Vector3};

use crate::random;

pub trait VectorAdditions {
    fn random(range: Range<f64>) -> Self;
    fn random_in_unit_sphere() -> Self;
    fn random_in_hemisphere(normal: Vector3<f64>) -> Self;
    fn random_in_unit_disk() -> Self;
    fn near_zero(&self) -> bool;
    fn reflect(&self, n: Vector3<f64>) -> Self;
    fn refract(&self, n: Vector3<f64>, etai_over_etat: f64) -> Self;
}

impl VectorAdditions for Vector3<f64> {
    #[inline]
    fn random(range: Range<f64>) -> Self {
        vec3(
            random(range.clone()),
            random(range.clone()),
            random(range.clone()),
        )
    }

    #[inline]
    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1. ..1.);
            if p.magnitude2() >= 1. {
                continue;
            }
            return p;
        }
    }

    #[inline]
    fn random_in_hemisphere(normal: Vector3<f64>) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0. {
            return in_unit_sphere;
        }
        return -in_unit_sphere;
    }

    #[inline]
    fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        return (self.x.abs() < EPS) && (self.y.abs() < EPS) && (self.z.abs() < EPS);
    }
    #[inline]
    fn reflect(&self, n: Vector3<f64>) -> Self {
        return self - (2. * self.dot(n)) * n;
    }

    #[inline]
    fn refract(&self, n: Vector3<f64>, etai_over_etat: f64) -> Self {
        let cos_theta = self.dot(-n).min(1.);
        let r_out_perp = etai_over_etat * (self + cos_theta * n);
        let r_out_parallel = -((1. - r_out_perp.magnitude2()).abs()).sqrt() * n;
        return r_out_perp + r_out_parallel;
    }

    fn random_in_unit_disk() -> Self {
        loop {
            let p = vec3(random(-1. ..1.), random(-1. ..1.), 0.);
            if p.magnitude2() >= 1. {
                continue;
            }
            return p;
        }
    }
}
