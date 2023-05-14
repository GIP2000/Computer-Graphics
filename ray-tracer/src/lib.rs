use cgmath::Vector3;
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    thread_rng, Rng,
};

pub mod camera;
pub mod image;
pub mod material;
pub mod ray;
pub mod vector_additon;

pub type Color = Vector3<f64>;

pub fn random<T: SampleUniform, R: SampleRange<T>>(range: R) -> T {
    thread_rng().gen_range(range)
}
