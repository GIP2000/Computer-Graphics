use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use cgmath::{vec3, Point3};
use ray_tracer::{
    camera::Camera,
    image::PPMImageWriter,
    material::{Dielectric, Lambertian, Metal},
    random_double,
    ray::hittable::{HittableList, Sphere},
};

fn main() -> Result<()> {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    let image = PPMImageWriter::new("image.ppm", IMAGE_WIDTH, ASPECT_RATIO, SAMPLES_PER_PIXEL)?;

    // World
    let material_ground = Rc::new(RefCell::new(Lambertian::new(vec3(0.8, 0.8, 0.))));
    let material_center = Rc::new(RefCell::new(Dielectric::new(1.5)));
    let material_left = Rc::new(RefCell::new(Dielectric::new(1.5)));
    let material_right = Rc::new(RefCell::new(Metal::new(vec3(0.8, 0.6, 0.2), 1.)));
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    )));
    // Camera
    let cam = Camera::new(Point3::new(0., 0., 0.), ASPECT_RATIO, 2., 1.);

    image.write(|j, i, w| {
        let mut pixel_color = vec3(0., 0., 0.);
        for _ in 0..w.samples_per_pixel {
            let u = (i as f64 + random_double(0. ..1.)) / (w.image_width - 1) as f64;
            let v = (j as f64 + random_double(0. ..1.)) / (w.image_height - 1) as f64;
            pixel_color += cam.get_ray(u, v).color(&world, MAX_DEPTH);
        }
        pixel_color
    })?;

    return Ok(());
}
