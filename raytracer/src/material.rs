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
}

pub struct Lambertian {
    albedo: Option<Arc<dyn Texture>>,
}

impl Lambertian {
    pub fn new1(a: &Vec3) -> Self {
        Self {
            albedo: Some(Arc::new(SolidColor::new(a.clone()))),
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
        let mut scatter_direction = rec.normal.clone() + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        *scattered = Ray::new(rec.point3.clone(), scatter_direction, _r_in.tm());
        *attenuation = self
            .albedo
            .clone()
            .unwrap()
            .value(rec.u, rec.v, &mut rec.point3);
        true
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
            albedo: a.clone(),
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
            rec.point3.clone(),
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
            r_in.tm,
        );
        *attenuation = self.albedo.clone();

        (scattered.direc() * rec.normal.clone()) > 0.0
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

        let cos_theta: f64 = if ((-unit_direction.clone()) * rec.normal.clone()) < 1.0 {
            (-unit_direction.clone()) * rec.normal.clone()
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

        *scattered = Ray::new(rec.point3.clone(), direction, r_in.tm);

        true
    }
}
