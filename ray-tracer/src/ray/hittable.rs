use cgmath::{vec3, InnerSpace, Point3, Vector3};

use super::Ray;

pub struct HitRecord {
    pub t: f64,
    pub p: Point3<f64>,
    pub normal: Vector3<f64>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Point3<f64>, outward_normal: Vector3<f64>, r: &Ray) -> Self {
        let mut hr = Self {
            t,
            p,
            normal: vec3(0., 0., 0.),
            front_face: false,
        };
        hr.set_face_normal(r, outward_normal);
        return hr;
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f64>) {
        self.front_face = r.dir.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_record = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = record.t;
                temp_record = Some(record);
            }
        }

        return temp_record;
    }
}

impl HittableList {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        let mut lst = Self::default();
        lst.add(object);
        return lst;
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Point3::new(0., 0., 0.),
            radius: 0.,
        }
    }
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.magnitude2();
        let half_b = oc.dot(r.dir);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        return Some(HitRecord::new(t, p, outward_normal, r));
    }
}
