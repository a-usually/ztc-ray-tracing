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
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let mut invd = 1.0 / r.direc().x();
        let mut t0 = (self.clone().mimimum.x() - r.ori().x()) * invd;
        let mut t1 = (self.clone().maximum.x() - r.ori().x()) * invd;

        if invd < 0.0 {
            std::mem::swap(&mut t1, &mut t0);
        }

        t_min = fmax(t0, t_min);
        t_max = fmin(t1, t_max);

        if t_max <= t_min {
            return false;
        }

        invd = 1.0 / r.direc().y();
        t0 = (self.clone().mimimum.y() - r.ori().y()) * invd;
        t1 = (self.clone().maximum.y() - r.ori().y()) * invd;

        if invd < 0.0 {
            std::mem::swap(&mut t1, &mut t0);
        }

        t_min = fmax(t0, t_min);
        t_max = fmin(t1, t_max);

        if t_max <= t_min {
            return false;
        }

        invd = 1.0 / r.direc().z();
        t0 = (self.clone().mimimum.z() - r.ori().z()) * invd;
        t1 = (self.clone().maximum.z() - r.ori().z()) * invd;

        if invd < 0.0 {
            std::mem::swap(&mut t1, &mut t0);
        }

        t_min = fmax(t0, t_min);
        t_max = fmin(t1, t_max);

        if t_max <= t_min {
            return false;
        }
        true
    }
    // pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
    //     for a in 0..3 {
    //         let inv_d = 1.0 / r.direc()[a];
    //         let mut t0 = ((*self).min()[a] - r.ori()[a]) * inv_d;
    //         let mut t1 = ((*self).max()[a] - r.ori()[a]) * inv_d;
    //         if inv_d < 0.0 {
    //             std::mem::swap(&mut t0, &mut t1);
    //         }
    //         t_min = t_min.max(t0);
    //         t_max = t_max.min(t1);
    //         if t_max <= t_min {
    //             return false;
    //         }
    //     }
    //     true
    // }

    pub fn min(&self) -> Vec3 {
        self.mimimum
    }
    pub fn max(&self) -> Vec3 {
        self.maximum
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
