pub use crate::bvh::AAbb;
pub use crate::vec3::Vec3;

use rand::Rng;
use std::f64::consts::PI;

//utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_i32() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen::<i32>()
}
pub fn random_i32_1(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_f64_1(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn fmax(x: f64, y: f64) -> f64 {
    if x >= y {
        x
    } else {
        y
    }
}

pub fn fmin(x: f64, y: f64) -> f64 {
    if x >= y {
        y
    } else {
        x
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
