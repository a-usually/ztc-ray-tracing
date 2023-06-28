pub use crate::aabb::AAbb;
pub use crate::hiitable::{Hiitable, HitRecord};
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

use std::f64::consts::PI;
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
            center: *c,
            radius: r,
            mat: m,
        }
    }

    pub fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hiitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.ori() - self.center;
        let a = r.direc().squared_length();
        let half_b = oc * r.direc();
        let c = oc.squared_length() - self.radius * self.radius;
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
        let outward_normal = (rec.point3 - self.center) / self.radius;
        rec.set_front_size(r, &outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AAbb) -> bool {
        *output_box = AAbb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
