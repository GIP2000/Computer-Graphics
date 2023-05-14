use anyhow::Result;
use cgmath::{vec3, Deg, ElementWise, EuclideanSpace, InnerSpace, Point3, Vector3};
use ray_tracer::{
    camera::Camera,
    image::PPMImageWriter,
    material::{Dielectric, Lambertian, Metal},
    random,
    ray::hittable::{HittableList, Sphere},
    vector_additon::VectorAdditions,
};
use rayon::prelude::*;

fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    let ground_material = Lambertian::new(vec3(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random(0. ..1.);
            let center = Point3::new(
                a as f64 + 0.9 * random(0. ..1.),
                0.2,
                b as f64 + 0.9 * random(0. ..1.),
            );
            if (center.to_vec() - vec3(4., 0.2, 0.)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Vector3<f64> =
                        Vector3::random(0. ..1.).mul_element_wise(Vector3::random(0. ..1.));
                    world.add(Box::new(Sphere::new(center, 0.2, Lambertian::new(albedo))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vector3::random(0. ..0.5);
                    let fuzz = random(0. ..0.5);
                    world.add(Box::new(Sphere::new(center, 0.2, Metal::new(albedo, fuzz))));
                } else {
                    // glass
                    world.add(Box::new(Sphere::new(center, 0.2, Dielectric::new(1.5))));
                }
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material1,
    )));
    let material2 = Lambertian::new(vec3(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material2,
    )));
    let material3 = Metal::new(vec3(0.7, 0.6, 0.5), 0.);
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material3,
    )));
    return world;
}

fn main() -> Result<()> {
    const ASPECT_RATIO: f64 = 3. / 2.;
    const IMAGE_WIDTH: u32 = 1200;
    const SAMPLES_PER_PIXEL: u32 = 500;
    const MAX_DEPTH: i32 = 50;

    let image = PPMImageWriter::new("image.ppm", IMAGE_WIDTH, ASPECT_RATIO, SAMPLES_PER_PIXEL)?;

    // World
    let world = random_scene();
    // Camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let cam = Camera::new(
        lookfrom,
        lookat,
        vec3(0., 1., 0.),
        Deg(20.).into(),
        ASPECT_RATIO,
        0.1,
        10.,
    );

    image.write(|j, i, w| {
        (0..w.samples_per_pixel)
            .into_par_iter()
            .fold(
                || vec3(0., 0., 0.),
                |acc, _| {
                    let u = (i as f64 + random(0. ..1.)) / (w.image_width - 1) as f64;
                    let v = (j as f64 + random(0. ..1.)) / (w.image_height - 1) as f64;
                    let color = cam.get_ray(u, v).color(&world, MAX_DEPTH);
                    acc + color
                },
            )
            .sum()
    })?;

    return Ok(());
}
