pub use crate::hiitable::HitRecord;
use crate::random_f64;
pub use crate::ray::Ray;
pub use crate::vec3::Vec3;

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
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: &Vec3) -> Self {
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
        *scattered = Ray::new(rec.point3.clone(), scatter_direction);
        *attenuation = self.albedo.clone();
        return true;
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
        let reflected = Vec3::reflect(&r_in.direc().unit().clone(), &rec.normal.clone());

        *scattered = Ray::new(
            rec.point3.clone(),
            reflected + Vec3::random_in_unit_sphere() * self.fuzz,
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
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
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
        let refraction_ratio: f64;
        if rec.front_size {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        };

        let unit_direction = r_in.direc.unit();

        let cos_theta: f64;
        if ((-unit_direction.clone()) * rec.normal.clone()) < 1.0 {
            cos_theta = (-unit_direction.clone()) * rec.normal.clone();
        } else {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = (refraction_ratio * sin_theta) > 1.0;
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64() {
            direction = Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.point3.clone(), direction);

        return true;
    }
}
