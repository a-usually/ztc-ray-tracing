pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use crate::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord{
    pub point3: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_size: bool,
    pub mat: Option<Arc<dyn Material>>,
}

impl HitRecord{
    pub fn new() -> Self{
        Self { point3: Vec3::new(0.0,0.0,0.0), normal: Vec3::new(0.0,0.0,0.0), t: (0.0), front_size: (true) , mat: (None)}
    }
    pub fn set_front_size(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_size = r.direc() * outward_normal.clone() < 0.0 ;
        if self.front_size == true {
            self.normal =  outward_normal.clone();
        }
        else {
            self.normal =  -outward_normal.clone();
        }
    }
}
pub trait Hiitable{
    fn hit(&self, r :&Ray, t_min :f64, t_max :f64, rec: &mut HitRecord) -> bool;
}

