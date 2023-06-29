pub use crate::aabb::AAbb;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::rtweekend::{degrees_to_radians, fmax, fmin};
pub use crate::vec3::Vec3;
use std::sync::Arc;

const INFINITY: f64 = f64::INFINITY;

#[derive(Clone)]
pub struct HitRecord {
    pub point3: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_size: bool,
    pub mat: Option<Arc<dyn Material>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point3: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: (0.0),
            u: (0.0),
            v: (0.0),
            front_size: (false),
            mat: (None),
        }
    }
    pub fn set_front_size(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_size = r.direc() * *outward_normal < 0.0;
        if self.front_size {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}

pub trait Hiitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AAbb) -> bool;
}

pub struct Rotatey {
    ptr: Option<Arc<dyn Hiitable>>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AAbb,
}

impl Rotatey {
    pub fn new(p: Option<Arc<dyn Hiitable>>, angle: f64) -> Self {
        let mut bbox_0 = AAbb::new_0();
        let radians = degrees_to_radians(angle);
        let sin_theta_0 = radians.sin();
        let cos_theta_0 = radians.cos();
        let hashbox0 = p.clone().unwrap().bounding_box(0.0, 1.0, &mut bbox_0);

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox_0.clone().max().x()
                        + (1 - i) as f64 * bbox_0.clone().min().x();
                    let y = j as f64 * bbox_0.clone().max().y()
                        + (1 - j) as f64 * bbox_0.clone().min().y();
                    let z = k as f64 * bbox_0.clone().max().z()
                        + (1 - k) as f64 * bbox_0.clone().min().z();

                    let newx = cos_theta_0 * x + sin_theta_0 * z;
                    let newz = -sin_theta_0 * x + cos_theta_0 * z;

                    let taster = Vec3::new(newx, y, newz);

                    min.x = fmin(min.x(), taster.x());
                    min.y = fmin(min.y(), taster.y());
                    min.z = fmin(min.z(), taster.z());

                    max.x = fmax(max.x(), taster.x());
                    max.y = fmax(max.y(), taster.y());
                    max.z = fmax(max.z(), taster.z());
                }
            }
        }
        bbox_0 = AAbb::new(min, max);
        Self {
            ptr: p,
            sin_theta: sin_theta_0,
            cos_theta: cos_theta_0,
            hasbox: hashbox0,
            bbox: bbox_0,
        }
    }
}

impl Hiitable for Rotatey {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.ori();
        let mut direction = r.direc();
        origin.x = self.cos_theta * r.ori().x() - self.sin_theta * r.ori().z();
        origin.z = self.sin_theta * r.ori().x() + self.cos_theta * r.ori().z();

        direction.x = self.cos_theta * r.direc().x() - self.sin_theta * r.direc().z();
        direction.z = self.sin_theta * r.direc().x() + self.cos_theta * r.direc().z();

        let rotated_r = Ray::new(origin, direction, r.tm());

        if !self.ptr.clone().unwrap().hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.point3;
        let mut normal = rec.normal;

        p.x = self.cos_theta * rec.point3.x() + self.sin_theta * rec.point3.z();
        p.z = -self.sin_theta * rec.point3.x() + self.cos_theta * rec.point3.z();

        normal.x = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
        normal.z = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

        rec.point3 = p;
        rec.set_front_size(&rotated_r, &normal);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AAbb) -> bool {
        *output_box = self.bbox.clone();
        self.hasbox
    }
}
