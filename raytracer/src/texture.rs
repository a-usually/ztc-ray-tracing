pub use crate::vec3::Vec3;

use std::sync::Arc;
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

#[derive(Clone)]
pub struct SolidColor {
    color_value: Vec3,
}

impl SolidColor {
    pub fn new_0() -> Self {
        Self {
            color_value: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new(c: Vec3) -> Self {
        Self {
            color_value: c.clone(),
        }
    }

    pub fn color_value(&self) -> Vec3 {
        self.color_value.clone()
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color_value.clone()
    }
}

pub struct CheckerTexture {
    odd: Option<Arc<dyn Texture>>,
    even: Option<Arc<dyn Texture>>,
}

impl CheckerTexture {
    pub fn new_1(_even: Option<Arc<dyn Texture>>, _odd: Option<Arc<dyn Texture>>) -> Self {
        Self {
            odd: _odd,
            even: _even,
        }
    }

    pub fn new_2(c1: Vec3, c2: Vec3) -> Self {
        Self {
            even: Some(Arc::new(SolidColor::new(c1))),
            odd: Some(Arc::new(SolidColor::new(c2))),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (p.x() * 10.0).sin() * (p.y() * 10.0).sin() * (p.z() * 10.0).sin();
        if sines < 0.0 {
            self.odd.clone().unwrap().value(u, v, p)
        } else {
            self.even.clone().unwrap().value(u, v, p)
        }
    }
}
