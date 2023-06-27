pub use crate::aabb::AAbb;
pub use crate::hiitable::{Hiitable, HitRecord};
pub use crate::hittable_list::HittableList;
pub use crate::material::Material;
pub use crate::moving_sphere::MovingSphere;
pub use crate::ray::Ray;
use crate::rtweekend::random_i32_1;

use std::cmp::Ordering::{Greater, Less};
use std::sync::Arc;

pub struct BVH_node {
    left: Option<Arc<dyn Hiitable>>,
    right: Option<Arc<dyn Hiitable>>,
    box_bvh: AAbb,
}

impl BVH_node {
    pub fn box_compare(
        a: &Option<Arc<dyn Hiitable>>,
        b: &Option<Arc<dyn Hiitable>>,
        axis: i32,
    ) -> bool {
        let mut box_a = AAbb::new_0();
        let mut box_b = AAbb::new_0();
        if !a.clone().unwrap().bounding_box(0.0, 0.0, &mut box_a)
            || !b.clone().unwrap().bounding_box(0.0, 0.0, &mut box_b)
        {
            println!("No bounding box in bvh_node constructor.\n");
        }
        box_a.min()[axis as usize] < box_b.min()[axis as usize]
    }

    pub fn box_x_compare(
        a: &Option<Arc<dyn Hiitable>>,
        b: &Option<Arc<dyn Hiitable>>,
    ) -> std::cmp::Ordering {
        if BVH_node::box_compare(a, b, 0) {
            return Less;
        }
        return Greater;
    }

    pub fn box_y_compare(
        a: &Option<Arc<dyn Hiitable>>,
        b: &Option<Arc<dyn Hiitable>>,
    ) -> std::cmp::Ordering {
        if BVH_node::box_compare(a, b, 1) {
            return Less;
        }
        return Greater;
    }

    pub fn box_z_compare(
        a: &Option<Arc<dyn Hiitable>>,
        b: &Option<Arc<dyn Hiitable>>,
    ) -> std::cmp::Ordering {
        if BVH_node::box_compare(a, b, 2) {
            return Less;
        }
        return Greater;
    }

    pub fn new(
        str_objects: &mut Vec<Option<Arc<dyn Hiitable>>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let objects = str_objects;
        let axis = random_i32_1(0, 2);
        let comparator = if axis == 0 {
            BVH_node::box_x_compare
        } else if axis == 1 {
            BVH_node::box_y_compare
        } else {
            BVH_node::box_z_compare
        };

        let object_span = end - start;
        let mut left_0: Option<Arc<dyn Hiitable>>;
        let mut right_0: Option<Arc<dyn Hiitable>>;
        if object_span == 1 {
            left_0 = objects[start].clone();
            right_0 = objects[start].clone();
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Less {
                left_0 = objects[start].clone();
                right_0 = objects[start + 1].clone();
            } else {
                left_0 = objects[start + 1].clone();
                right_0 = objects[start].clone();
            }
        } else {
            objects.as_mut_slice()[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left_0 = Some(Arc::new(BVH_node::new(
                objects,
                start,
                mid,
                time0.clone(),
                time1.clone(),
            )));
            right_0 = Some(Arc::new(BVH_node::new(
                objects,
                mid,
                end,
                time0.clone(),
                time1.clone(),
            )))
        }

        let mut box_left = AAbb::new_0();
        let mut box_right = AAbb::new_0();

        if !left_0
            .clone()
            .unwrap()
            .bounding_box(time0, time1, &mut box_left)
            || !right_0
                .clone()
                .unwrap()
                .bounding_box(time0, time1, &mut box_right)
        {
            println!("No bounding box in bvh_node constructor.\n");
        }

        Self {
            left: left_0,
            right: right_0,
            box_bvh: AAbb::surrounding_box(&box_left, &box_right),
        }
    }
}

impl Hiitable for BVH_node {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.box_bvh.clone().hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = self.clone().left.clone().unwrap().hit(r, t_min, t_max, rec);
        let hit_right = if hit_left {
            self.clone()
                .right
                .clone()
                .unwrap()
                .hit(r, t_min, rec.clone().t, rec)
        } else {
            self.clone()
                .right
                .clone()
                .unwrap()
                .hit(r, t_min, t_max, rec)
        };

        hit_left || hit_right
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AAbb) -> bool {
        *output_box = self.box_bvh.clone();
        true
    }
}
