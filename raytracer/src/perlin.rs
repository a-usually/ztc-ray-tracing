use crate::rtweekend::{random_i32_1,random_f64};
pub use crate::vec3::Vec3;

use std::vec::Vec;
pub struct Perlin {
    point_count: i32,
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new_0() -> Self {
        Self {
            point_count: 256,
            ranfloat: vec![0.0; 0],
            perm_x: vec![0;0],
            perm_y: vec![0;0],
            perm_z: vec![0;0],
        }
    }
    pub fn new(&self) -> Self {
        let mut ranfloat_0: Vec<f64> = vec![0.0; self.point_count as usize];
        for i in 0..self.point_count.clone() {
            ranfloat_0[i as usize] = random_f64();
        }
        Self {
            point_count: 256,
            ranfloat: ranfloat_0,
            perm_x: self.perlin_generate_perm(),
            perm_y: self.perlin_generate_perm(),
            perm_z: self.perlin_generate_perm(),
        }
    }

    pub fn perlin_generate_perm(&self) -> Vec<i32> {
        let mut p = vec![0; self.point_count as usize];

        for i in 0..self.point_count.clone() {
            p[i as usize] = i;
        }

        Perlin::permute(&mut p, self.point_count);

        p
    }

    pub fn permute(p: &mut Vec<i32>, n: i32) {
        for i in 1..n {
            let target = random_i32_1(0, n - i) as usize;
            let tmp = p[i as usize];
            p[i as usize] = p[target];
            p[target] = tmp;
        }
    }

    pub fn noise(&self, p: &Vec3) -> f64{
        let i = (((p.x() * 4.0) as i32) & 255) as usize;
        let j = (((p.y() * 4.0) as i32) & 255) as usize;
        let k = (((p.z() * 4.0) as i32) & 255) as usize;
        
        self.ranfloat[self.perm_x[i] as usize ^ self.perm_y[j] as usize ^ self.perm_z[k] as usize] 
    }
}

