pub use crate::aabb::AAbb;
pub use crate::aarect::{Xyrect, Xzrect, Yzrect};
pub use crate::hiitable::Hiitable;
pub use crate::hittable_list::HittableList;
pub use crate::material::Material;
pub use crate::vec3::Vec3;

use std::sync::Arc;
pub struct Box {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
}

impl Box {
    pub fn new_0() -> Self {
        Self {
            box_min: Vec3::zero(),
            box_max: Vec3::zero(),
            sides: HittableList::new(),
        }
    }

    pub fn new(p0: Vec3, p1: Vec3, ptr: Option<Arc<dyn Material>>) -> Self {
        let mut sides_0 = HittableList::new();
        sides_0.add(Some(Arc::new(Xyrect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            ptr.clone(),
        ))));
        sides_0.add(Some(Arc::new(Xyrect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            ptr.clone(),
        ))));
        sides_0.add(Some(Arc::new(Xzrect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            ptr.clone(),
        ))));
        sides_0.add(Some(Arc::new(Xzrect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            ptr.clone(),
        ))));
        sides_0.add(Some(Arc::new(Yzrect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            ptr.clone(),
        ))));
        sides_0.add(Some(Arc::new(Yzrect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            ptr.clone(),
        ))));

        Self {
            box_min: p0,
            box_max: p1,
            sides: sides_0,
        }
    }
}

impl Hiitable for Box {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AAbb) -> bool {
        *output_box = AAbb::new(self.box_min, self.box_max);
        true
    }

    fn hit(
        &self,
        r: &crate::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::material::HitRecord,
    ) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }
}
