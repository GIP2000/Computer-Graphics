use anyhow::Result;
use cgmath::{vec3, Point3};
use rand::{distributions::Uniform, thread_rng, Rng};
use ray_tracer::{
    camera::Camera,
    image::PPMImageWriter,
    ray::hittable::{HittableList, Sphere},
};

fn random_double() -> f64 {
    let mut rng = thread_rng();
    let uniform = Uniform::new(0., 1.);
    return rng.sample(uniform);
}

fn main() -> Result<()> {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const SAMPLES_PER_PIXEL: u32 = 100;

    let image = PPMImageWriter::new("image.ppm", IMAGE_WIDTH, ASPECT_RATIO, SAMPLES_PER_PIXEL)?;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    // Camera
    let cam = Camera::new(Point3::new(0., 0., 0.), ASPECT_RATIO, 2., 1.);

    image.write(|j, i, w| {
        let mut pixel_color = vec3(0., 0., 0.);
        for _ in 0..w.samples_per_pixel {
            let u = (i as f64 + random_double()) / (w.image_width - 1) as f64;
            let v = (j as f64 + random_double()) / (w.image_height - 1) as f64;
            pixel_color += cam.get_ray(u, v).color(&world);
        }
        pixel_color
    })?;

    return Ok(());
}
