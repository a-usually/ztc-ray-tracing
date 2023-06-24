pub use crate::vec3::Vec3;

pub struct Ray {
    pub direc: Vec3,
    pub ori: Vec3,
}

impl Ray {
    pub fn info(&self) {
        println!("ray{}", self.direc.x());
    }
}
