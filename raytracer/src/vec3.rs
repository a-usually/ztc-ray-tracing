pub use crate::rtweekend::{random_f64, random_f64_1};
pub use std::ops::{Add, AddAssign, Div, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(&self) -> Self {
        if self.length() == 0.0 {
            println!("error vector");
        }
        Self {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }

    pub fn elemul(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Self {
            x: v1.x * v2.x,
            y: v1.y * v2.y,
            z: v1.z * v2.z,
        }
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Self {
            x: v1.y * v2.z - v2.y * v1.z,
            y: v1.z * v2.x - v2.z * v1.x,
            z: v1.x * v2.y - v2.x * v1.y,
        }
    }

    pub fn random_vec3_1() -> Vec3 {
        Vec3::new(random_f64(), random_f64(), random_f64())
    }

    pub fn random_vec3_2(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_f64_1(min, max),
            random_f64_1(min, max),
            random_f64_1(min, max),
        )
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_vec3_2(-1.0, 1.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere * *normal > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut p;
        loop {
            p = Vec3::new(random_f64_1(-1.0, 1.0), random_f64_1(-1.0, 1.0), 0.0);
            if p.clone().squared_length() < 1.0 {
                return p;
            };
        }
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-7;
        (self.x < s)
            && (self.x > -s)
            && (self.y < s)
            && (self.y > -s)
            && (self.z < s)
            && (self.z > -s)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * (*v * *n) * 2.0
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta: f64 = if ((-*uv) * *n) < 1.0 {
            (-*uv) * *n
        } else {
            1.0
        };
        let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = if 1.0 > r_out_perp.squared_length() {
            *n * (-(1.0 - r_out_perp.squared_length()).sqrt())
        } else {
            *n * (-(r_out_perp.squared_length() - 1.0).sqrt())
        };
        r_out_perp + r_out_parallel
    }

    //    pub fn info(&self){
    //     println!("x:{},y:{},z:{}",self.x,self.y,self.z);
    //    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        };
    }
}

impl Mul for Vec3 {
    type Output = f64;
    fn mul(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("error!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }
    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }
    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }
    #[test]
    fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }
    // #[test]
    // fn test_add_assign_f64() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x += 233.0;
    //     assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    // }
    // #[test]
    // fn test_sub() {
    //     assert_eq!(
    //         Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
    //         Vec3::new(-1.0, -4.0, -7.0)
    //     )
    // }
    // #[test]
    // fn test_sub_assign() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x -= Vec3::new(2.0, 4.0, 6.0);
    //     assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    // }
    // #[test]
    // fn test_sub_f64() {
    //     assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    // }
    // #[test]
    // fn test_sub_assign_f64() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x -= 1.0;
    //     assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    // }
    // #[test]
    // fn test_mul() {
    //     assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    // }
    // #[test]
    // fn test_mul_assign() {
    //     let mut x = Vec3::new(1.0, 0.0, -1.0);
    //     x *= 2.0;
    //     assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    // }
    // #[test]
    // fn test_mul_f64() {
    //     assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    // }
    // #[test]
    // fn test_div() {
    //     assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    // }
    // #[test]
    // fn test_elemul() {
    //     assert_eq!(
    //         Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
    //         Vec3::new(1.0, 4.0, 9.0)
    //     );
    // }
    // #[test]
    // fn test_cross() {
    //     assert_eq!(
    //         Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
    //         Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
    //     );
    // }
    // #[test]
    // fn test_neg() {
    //     assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    // }
    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0);
    }
    // #[test]
    // fn test_length() {
    //     assert_eq!(
    //         Vec3::new(3.0, 4.0, 5.0).length(),
    //         ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
    //     );
    // }
    // #[test]
    // fn test_unit() {
    //     assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
    //     assert_eq!(
    //         Vec3::new(-233.0, 0.0, 0.0).unit(),
    //         Vec3::new(-1.0, 0.0, 0.0)
    //     );
    // }
    // #[test]
    // #[should_panic]
    // fn test_unit_panic() {
    //     Vec3::new(0.0, 0.0, 0.0).unit();
    // }
}
