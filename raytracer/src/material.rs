pub use crate::hiitable::HitRecord;
pub use crate::moving_sphere::MovingSphere;
use crate::random_f64;
pub use crate::ray::Ray;
pub use crate::texture::SolidColor;
use crate::texture::Texture;
pub use crate::vec3::Vec3;

use std::sync::Arc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f64,_v: f64, _p: &Vec3) -> Vec3;
}

pub struct Lambertian {
    albedo: Option<Arc<dyn Texture>>,
}

impl Lambertian {
    pub fn new1(a: &Vec3) -> Self {
        Self {
            albedo: Some(Arc::new(SolidColor::new(*a))),
        }
    }

    pub fn new2(a: &Option<Arc<dyn Texture>>) -> Self {
        Self { albedo: a.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.point3, scatter_direction, _r_in.tm());
        *attenuation = self
            .albedo
            .clone()
            .unwrap()
            .value(rec.u, rec.v, &rec.point3);
        true
    }

    fn emitted(&self, _u: f64,_v: f64, _p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

//metal
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: &Vec3, num: f64) -> Self {
        let mut b = 1.0;
        if num < 1.0 {
            b = num;
        }
        Self {
            albedo: *a,
            fuzz: b,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direc().unit(), &rec.normal.clone());

        *scattered = Ray::new(
            rec.point3,
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.tm,
        );
        *attenuation = self.albedo;

        (scattered.direc() * rec.normal) > 0.0
    }

    fn emitted(&self, _u: f64,_v: f64, _p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

//dielectric
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = if rec.front_size {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direc.unit();

        let cos_theta: f64 = if ((-unit_direction) * rec.normal) < 1.0 {
            (-unit_direction) * rec.normal
        } else {
            1.0
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = (refraction_ratio * sin_theta) > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.point3, direction, r_in.tm);

        true
    }

    fn emitted(&self, _u: f64,_v: f64, _p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct DiffLight {
    emit: Option<Arc<dyn Texture>>,
}

impl DiffLight {
    pub fn new1(a: Option<Arc<dyn Texture>>) -> Self {
        Self {
            emit: a,
        }
    }

    pub fn new2(c: Vec3) -> Self {
        Self{
            emit: Some(Arc::new(SolidColor::new(c)))
        }
    }
}

impl Material for DiffLight {
    fn scatter(
            &self,
            _r_in: &Ray,
            _rec: &mut HitRecord,
            _attenuation: &mut Vec3,
            _scattered: &mut Ray,
        ) -> bool {
        false
    }

    fn emitted(&self, u: f64,v: f64, p: &Vec3) -> Vec3 {
        self.emit.clone().unwrap().value(u, v, p)
    }
}