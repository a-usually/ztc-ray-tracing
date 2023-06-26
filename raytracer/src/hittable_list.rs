use crate::hiitable::{HitRecord,Hiitable};
use crate::ray::Ray;
use crate::object::Sphere;
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Option<Arc<dyn Hiitable>>>,
}

//impl Default for Hittable_list {
  //  fn default() -> Self {
    //    Self::new();
   // }
//}

impl HittableList {
    pub fn new()-> Self {
        Self { objects: Vec::new(), }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool{
        let mut temp_rec = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;
    
        for object in (*self).clone().objects {
            if object.clone().unwrap().hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }
    pub fn add(&mut self, object: Option<Arc<dyn Hiitable>>) {
        self.objects.push(object);
    }
}
