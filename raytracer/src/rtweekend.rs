use std::f64::consts::PI;
use rand::Rng;

//utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_f64_1(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}