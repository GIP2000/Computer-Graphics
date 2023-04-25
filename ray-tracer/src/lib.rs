use std::ops::Range;

use cgmath::Vector3;
use rand::{distributions::Uniform, thread_rng, Rng};

pub mod camera;
pub mod image;
pub mod material;
pub mod ray;
mod vector_additon;

pub type Color = Vector3<f64>;

pub fn random_double(range: Range<f64>) -> f64 {
    let mut rng = thread_rng();
    let uniform = Uniform::from(range);
    return rng.sample(uniform);
}
