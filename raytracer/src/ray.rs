pub use crate::vec3::Vec3;

pub struct Ray {
    pub direc: Vec3,
    pub ori: Vec3,
}

impl Ray {
    // pub fn info(&self) {
    //     println!("ray{}", self.direc.x());
    // }

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            ori: Vec3::new(origin.x, origin.y, origin.z),
            direc: Vec3::new(direction.x, direction.y, direction.z),
        }
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
    pub fn info(&self){
        println!("ori");
        self.ori.info();
        println!("dir");
        self.direc.info();
    }
}
