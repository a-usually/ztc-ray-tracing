pub use crate::aabb::AAbb;
pub use crate::hiitable::Hiitable;
pub use crate::material::Material;
pub use crate::vec3::Vec3;

use std::sync::Arc;

pub struct Xyrect {
    mp: Option<Arc<dyn Material>>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl Xyrect {
    pub fn new(
        _x0: f64,
        _x1: f64,
        _y0: f64,
        _y1: f64,
        _k: f64,
        _mp: Option<Arc<dyn Material>>,
    ) -> Self {
        Self {
            mp: _mp,
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }
}

impl Hiitable for Xyrect {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AAbb) -> bool {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        *output_box = AAbb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }

    fn hit(
        &self,
        r: &crate::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::material::HitRecord,
    ) -> bool {
        let t = (self.k - r.ori().z()) / r.direc().z();
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.ori().x() + t * r.direc().x();
        let y = r.ori().y() + t * r.direc().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_front_size(r, &outward_normal);
        rec.mat = self.mp.clone();
        rec.point3 = r.at(t);

        true
    }
}
