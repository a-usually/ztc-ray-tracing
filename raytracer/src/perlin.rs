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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.0 ; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranfloat[self.perm_x[(i + di) as usize & 255] as usize ^ self.perm_y[(j + dj) as usize & 255] as usize ^ self.perm_z[(k + dk) as usize & 255] as usize];
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64  * u + (1 - i) as f64 * (1.0 - u) as f64) *
                    (j as f64 * v + (1 - j) as f64 * (1.0 - v) as f64) *
                    (k as f64 * w + (1 - k) as f64 * (1.0 - w) as f64) * c[i][j][k];
                }
            }
        }

        accum
    }
}
