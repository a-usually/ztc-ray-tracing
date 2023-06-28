use crate::ray::Ray;
use crate::rtweekend::{fmax, fmin};
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct AAbb {
    pub mimimum: Vec3,
    pub maximum: Vec3,
}

impl AAbb {
    pub fn new_0() -> Self {
        Self {
            mimimum: Vec3::new(0.0, 0.0, 0.0),
            maximum: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self {
            mimimum: a,
            maximum: b,
        }
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            if a == 0 {
                let invd = 1.0 / r.direc().x();
                let mut t0 = fmin(
                    (self.mimimum.x() - r.ori.x()) / r.direc().x,
                    (self.maximum.x() - r.ori().x()) * invd,
                );
                let mut t1 = fmax(
                    (self.mimimum.x() - r.ori.x()) / r.direc().x,
                    (self.maximum.x() - r.ori().x()) * invd,
                );

                if invd < 0.0 {
                    std::mem::swap(&mut t1, &mut t0);
                }

                let t_min_0 = fmax(t0, t_min);
                let t_max_0 = fmin(t1, t_max);
                if t_max_0 <= t_min_0 {
                    return false;
                }
            } else if a == 1 {
                let invd = 1.0 / r.direc().y();
                let mut t0 = fmin(
                    (self.mimimum.y() - r.ori.y()) / r.direc().y,
                    (self.maximum.y() - r.ori().y()) * invd,
                );
                let mut t1 = fmax(
                    (self.mimimum.y() - r.ori.y()) / r.direc().y,
                    (self.maximum.y() - r.ori().y()) * invd,
                );

                if invd < 0.0 {
                    std::mem::swap(&mut t1, &mut t0);
                }

                let t_min_0 = fmax(t0, t_min);
                let t_max_0 = fmin(t1, t_max);
                if t_max_0 <= t_min_0 {
                    return false;
                }
            } else {
                let invd = 1.0 / r.direc().z();
                let mut t0 = fmin(
                    (self.mimimum.z() - r.ori.z()) / r.direc().z,
                    (self.maximum.z() - r.ori().z()) * invd,
                );
                let mut t1 = fmax(
                    (self.mimimum.z() - r.ori.z()) / r.direc().z,
                    (self.maximum.z() - r.ori().z()) * invd,
                );

                if invd < 0.0 {
                    std::mem::swap(&mut t1, &mut t0);
                }

                let t_min_0 = fmax(t0, t_min);
                let t_max_0 = fmin(t1, t_max);
                if t_max_0 <= t_min_0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn min(&self) -> Vec3 {
        self.mimimum.clone()
    }
    pub fn max(&self) -> Vec3 {
        self.maximum.clone()
    }

    pub fn surrounding_box(box0: &AAbb, box1: &AAbb) -> AAbb {
        let small = Vec3::new(
            fmin(box0.min().x(), box1.min().x()),
            fmin(box0.min().y(), box1.min().y()),
            fmin(box0.min().z(), box1.min().z()),
        );
        let big = Vec3::new(
            fmax(box0.max().x(), box1.max().x()),
            fmax(box0.max().y(), box1.max().y()),
            fmax(box0.max().z(), box1.max().z()),
        );
        AAbb::new(small, big)
    }
}
