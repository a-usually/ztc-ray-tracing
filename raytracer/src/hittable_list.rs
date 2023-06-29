pub use crate::aabb::AAbb;
pub use crate::hiitable::{Hiitable, HitRecord};
pub use crate::ray::Ray;

use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Option<Arc<dyn Hiitable>>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for object in (*self).clone().objects {
            if object
                .clone()
                .unwrap()
                .hit(r, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }

    pub fn add(&mut self, object: Option<Arc<dyn Hiitable>>) {
        self.objects.push(object);
    }

    pub fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AAbb) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box: AAbb = AAbb::new_0();
        let mut first_box = true;
        for object in (*self).clone().objects {
            if object
                .clone()
                .unwrap()
                .bounding_box(time0, time1, &mut temp_box)
            {
                return false;
            }
            *output_box = if first_box {
                temp_box.clone()
            } else {
                AAbb::surrounding_box(output_box, &temp_box)
            };
            first_box = false;
        }
        true
    }
}
