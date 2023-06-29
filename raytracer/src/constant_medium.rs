pub use crate::hiitable::Hiitable;
pub use crate::hittable_list::HitRecord;
pub use crate::material::{Material, Isotropic};
use crate::random_f64;
pub use crate::texture::Texture;
pub use crate::vec3::Vec3;

use std::sync::Arc;

const INFINITY: f64 = f64::INFINITY;

pub struct ConstantMedium {
    boundary: Option<Arc<dyn Hiitable>>,
    phase_function: Option<Arc<dyn Material>>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new1(b: Option<Arc<dyn Hiitable>>, d: f64, a: Option<Arc<dyn Texture>>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Some(Arc::new(Isotropic::new2(a)))
        }
    }
    pub fn new2(b: Option<Arc<dyn Hiitable>>, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Some(Arc::new(Isotropic::new1(c))),
        }
    }
}

impl Hiitable for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::r#box::AAbb) -> bool {
        self.boundary.clone().unwrap().bounding_box(time0, time1, output_box)
    }

    fn hit(&self, r: &crate::Ray, t_min: f64, t_max: f64, rec: &mut crate::material::HitRecord) -> bool {
        let enabledebug = false;
        let debugging = enabledebug && random_f64() < 0.00001;

        let mut rec1: HitRecord = HitRecord::new();
        let mut rec2: HitRecord = HitRecord::new();

        if !self.boundary.clone().unwrap().hit(r, -INFINITY, INFINITY, &mut rec1.clone()) {
            return false;
        }

        if !self.boundary.clone().unwrap().hit(r, rec1.t + 0.0001, INFINITY, &mut rec2.clone()) {
            return false;
        }

        if debugging {
            println!("\nt_min={},\nt_max={}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }

        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direc().length();
        let distance_inside_boundary = (rec2.t -rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance;
        rec.point3 = r.at(rec.t);

        if debugging {
            println!("hit_distance = {}\n",hit_distance);
            println!("rec.t = {}\n",rec.t);
            println!("rec.p.x = {},rec.p.y = {}. rec.p.z = {}.",rec.point3.x(),rec.point3.y(),rec.point3.z());
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_size = true;
        rec.mat = self.phase_function.clone();

        true
    }
}