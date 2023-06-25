pub use crate::vec3::Vec3;

pub struct Ray {
    pub direc: Vec3,
    pub ori: Vec3,
}

impl Ray {
    pub fn info(&self) {
        println!("ray{}", self.direc.x());
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.ori.clone() + self.direc.clone() * t
    }

    pub fn ori(&self) -> Vec3 {
        self.ori.clone()
    }

    pub fn direc(&self) -> Vec3 {
        self.direc.clone()
    }
}
