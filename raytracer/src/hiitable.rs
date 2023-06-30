pub use crate::aabb::AAbb;
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use std::sync::Arc;

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
pub trait Hiitable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AAbb) -> bool;
}
