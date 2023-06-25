pub use crate::vec3::Vec3;
pub use crate::hiitable::{Hiitable,HitRecord};
pub use crate::ray::Ray;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere{
    pub fn new(cen: Vec3, r: f64, ) -> Self {
        Self {
            center: cen,
            radius: r,
        }
    }
}


impl Hiitable for Sphere{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.ori() - self.center.clone();
        let a = r.direc().squared_length();
        let half_b = oc.clone() * r.direc();
        let c = oc.clone().squared_length() - self.radius * self.radius;
        let discriminant = half_b *half_b - a * c;
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
        rec.set_front_size(r,&outward_normal);

        return true;
    }
}