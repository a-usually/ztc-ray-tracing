pub use crate::aabb::AAbb;
pub use crate::hiitable::{Hiitable, HitRecord};
pub use crate::material::Material;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;
use std::sync::Arc;

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time_0: f64,
    pub time_1: f64,
    pub radius: f64,
    pub mat: Option<Arc<dyn Material>>,
}

impl MovingSphere {
    pub fn new_0() -> Self {
        Self {
            center0: Vec3::new(0.0, 0.0, 0.0),
            center1: Vec3::new(0.0, 0.0, 0.0),
            time_0: (0.0),
            time_1: (0.0),
            radius: (0.0),
            mat: (None),
        }
    }

    pub fn new(
        cen0: Vec3,
        cen1: Vec3,
        t_0: f64,
        t_1: f64,
        r: f64,
        _mat_0: Option<Arc<dyn Material>>,
    ) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time_0: t_0,
            time_1: t_1,
            radius: r,
            mat: _mat_0,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center0.clone()
            + (self.center1.clone() - self.center0.clone()) * (time - self.time_0)
                / (self.time_1 - self.time_0)
    }
}

impl Hiitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.ori() - self.center(r.tm());
        let a = r.direc().squared_length();
        let half_b = oc.clone() * r.direc();
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        //find the neareast root in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point3 = r.at(rec.t);
        rec.normal = (rec.point3.clone() - self.center(r.tm())) / self.radius;
        let outward_normal = (rec.point3.clone() - self.center(r.tm())) / self.radius;
        rec.set_front_size(r, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AAbb) -> bool {
        let box0 = AAbb::new(
            self.center(time0)
                - Vec3::new(
                    self.radius,
                    self.radius,
                    self.radius,
                ),
            self.center(time0)
                + Vec3::new(
                    self.radius,
                    self.radius,
                    self.radius,
                ),
        );
        let box1 = AAbb::new(
            self.center(time1)
                - Vec3::new(
                    self.radius,
                    self.radius,
                    self.radius,
                ),
            self.center(time1)
                + Vec3::new(
                    self.radius,
                    self.radius,
                    self.radius,
                ),
        );
        *output_box = AAbb::surrounding_box(&box0, &box1);
        true
    }
}
