use crate::rtweekend::{random_f64, random_i32_1};
pub use crate::vec3::Vec3;

use std::vec::Vec;
pub struct Perlin {
    // point_count: i32,
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Perlin {
    pub fn new_0() -> Self {
        Self {
            //point_count: 256,
            ranfloat: vec![0.0; 0],
            perm_x: vec![0; 0],
            perm_y: vec![0; 0],
            perm_z: vec![0; 0],
        }
    }
    pub fn new() -> Self {
        let point_count_0 = 256;
        let mut ranfloat_0: Vec<f64> = vec![0.0; point_count_0 as usize];
        for i in 0..point_count_0 {
            ranfloat_0[i as usize] = random_f64();
        }
        Self {
            //point_count: point_count_0,
            ranfloat: ranfloat_0,
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
        }
    }

    pub fn perlin_generate_perm() -> Vec<i32> {
        let point_count_0 = 256;
        let mut p = vec![0; point_count_0 as usize];

        for i in 0..point_count_0 {
            p[i as usize] = i;
        }

        Perlin::permute(&mut p, point_count_0);

        p
    }

    pub fn permute(p: &mut [i32], n: i32) {
        for i in 1..n {
            let target = random_i32_1(0, n - i) as usize;
            p.swap(i as usize, target);
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64 {
        let i = (((p.x() * 4.0) as i32) & 255) as usize;
        let j = (((p.y() * 4.0) as i32) & 255) as usize;
        let k = (((p.z() * 4.0) as i32) & 255) as usize;

        self.ranfloat[self.perm_x[i] as usize ^ self.perm_y[j] as usize ^ self.perm_z[k] as usize]
    }
}
