pub use crate::color::clamp;
pub use crate::perlin::Perlin;
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
    // pub fn new_0() -> Self {
    //     Self {
    //         color_value: Vec3::new(0.0, 0.0, 0.0),
    //     }
    // }

    pub fn new(c: Vec3) -> Self {
        Self { color_value: c }
    }

    // pub fn color_value(&self) -> Vec3 {
    //     self.color_value.clone()
    // }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color_value
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

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new_0(sc: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        //Vec3::new(1.0, 1.0, 1.0) * self.noise.turb(&(p.clone() * self.scale), 7)
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + self.noise.turb(p, 7) * 10.0).sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytes_per_scanline: i32,
}

impl ImageTexture {
    pub fn new_0() -> Self {
        Self {
            data: vec![0_u8; 0],
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        }
    }

    pub fn new(filename: &str) -> Self {
        let photo = image::open(filename).unwrap();

        Self {
            data: photo.clone().into_bytes(),
            width: photo.width() as i32,
            height: photo.height() as i32,
            bytes_per_scanline: (photo.width() * 3) as i32,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Vec3) -> Vec3 {
        if self.data == [0_u8, 0] {
            return Vec3::new(0.0, 1.0, 1.0);
        }

        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);

        let mut i = (u * self.width as f64) as i32;
        let mut j = (v * self.height as f64) as i32;
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let mut pixel: [f64; 3] = [0.0; 3];
        pixel[0] = self.data[(j * self.bytes_per_scanline + i * 3) as usize] as f64;
        pixel[1] = self.data[(j * self.bytes_per_scanline + i * 3 + 1) as usize] as f64;
        pixel[2] = self.data[(j * self.bytes_per_scanline + i * 3 + 2) as usize] as f64;

        Vec3::new(
            color_scale * pixel[0],
            color_scale * pixel[1],
            color_scale * pixel[2],
        )
    }
}
