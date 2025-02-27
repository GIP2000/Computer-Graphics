use std::{cell::RefCell, rc::Rc};

use cgmath::{vec3, InnerSpace, Vector3};

use crate::{
    random,
    ray::{hittable::HitRecord, Ray},
    vector_additon::VectorAdditions,
    Color,
};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Box<Self> {
        return Box::new(Self { albedo });
    }
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vector3::random_in_unit_sphere().normalize();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        return Some((self.albedo, Ray::new(rec.p, scatter_direction)));
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Box<Self> {
        return Box::new(Self {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        });
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().normalize().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + (self.fuzz * Vector3::random_in_unit_sphere()),
        );
        if scattered.direction().dot(rec.normal) > 0. {
            return Some((self.albedo, scattered));
        }
        return None;
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Box<Self> {
        return Box::new(Self { ir });
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        return r0 + (1. - r0) * (1. - cosine).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = vec3(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction().normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_reflect = refraction_ratio * sin_theta > 1.;
        let direction =
            if cannot_reflect || Self::reflectance(cos_theta, refraction_ratio) > random(0. ..1.) {
                unit_direction.reflect(rec.normal)
            } else {
                unit_direction.refract(rec.normal, refraction_ratio)
            };
        return Some((attenuation, Ray::new(rec.p, direction)));
    }
}
