use crate::rtweekend::random_i32_1;
pub use crate::vec3::Vec3;

use std::vec::Vec;
pub struct Perlin {
    // point_count: i32,
    ranvec: Vec<Vec3>,
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
            ranvec: vec![Vec3::new(0.0, 0.0, 0.0); 0],
            perm_x: vec![0; 0],
            perm_y: vec![0; 0],
            perm_z: vec![0; 0],
        }
    }
    pub fn new() -> Self {
        let point_count_0 = 256;
        let mut ranvec_0: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); point_count_0 as usize];
        for i in 0..point_count_0 {
            ranvec_0[i as usize] = Vec3::unit(&Vec3::random_vec3_2(-1.0, 1.0))
        }
        Self {
            //point_count: point_count_0,
            ranvec: ranvec_0,
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

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] =
                        self.ranvec[self.perm_x[(i + di) as usize & 255] as usize
                            ^ self.perm_y[(j + dj) as usize & 255] as usize
                            ^ self.perm_z[(k + dk) as usize & 255] as usize];
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let vec_loop = [0, 1];
        for i in vec_loop {
            for j in vec_loop {
                for k in vec_loop {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * (c[i][j][k] * weight_v)
                }
            }
        }

        accum
    }

    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        let mut i = 0;
        while i < depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
            i += 1;
        }

        accum.abs()
    }
}
