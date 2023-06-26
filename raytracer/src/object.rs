pub use crate::hiitable::{Hiitable, HitRecord};
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat: Option<Arc<dyn Material>>,
}

impl Sphere {
    pub fn new(c: &Vec3, r: f64, m: Option<Arc<dyn Material>>) -> Self {
        Self {
            center: c.clone(),
            radius: r,
            mat: m,
        }
    }
}

impl Hiitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.ori() - self.center.clone();
        let a = r.direc().squared_length();
        let half_b = oc.clone() * r.direc();
        let c = oc.clone().squared_length() - self.radius.clone() * self.radius.clone();
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        //find the neareast root
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point3 = r.at(rec.t);
        rec.normal = (rec.point3.clone() - self.center.clone()) / self.radius;
        let outward_normal = (rec.point3.clone() - self.center.clone()) / self.radius;
        rec.set_front_size(r, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}
