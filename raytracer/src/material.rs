use crate::aabb::AAbb;
pub use crate::hiitable::Hiitable;
pub use crate::hiitable::HitRecord;
pub use crate::moving_sphere::MovingSphere;
use crate::random_f64;
pub use crate::ray::Ray;
pub use crate::rtweekend::degrees_to_radians;
use crate::rtweekend::{fmax, fmin};
pub use crate::texture::SolidColor;
use crate::texture::Texture;
pub use crate::vec3::Vec3;

const INFINITY: f64 = f64::INFINITY;

use std::sync::Arc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3;
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

    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
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

    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
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

    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct DiffLight {
    emit: Option<Arc<dyn Texture>>,
}

impl DiffLight {
    pub fn new1(a: Option<Arc<dyn Texture>>) -> Self {
        Self { emit: a }
    }

    pub fn new2(c: Vec3) -> Self {
        Self {
            emit: Some(Arc::new(SolidColor::new(c))),
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

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.clone().unwrap().value(u, v, p)
    }
}

pub struct Translate {
    ptr: Option<Arc<dyn Hiitable>>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Option<Arc<dyn Hiitable>>, displacement: Vec3) -> Self {
        Self {
            ptr: p,
            offset: displacement,
        }
    }
}

impl Hiitable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray::new(r.ori() - self.offset, r.direc, r.tm());
        if !self.ptr.clone().unwrap().hit(&moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.point3 += self.offset;
        rec.clone().set_front_size(&moved_r, &rec.normal);

        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AAbb) -> bool {
        if !self
            .ptr
            .clone()
            .unwrap()
            .bounding_box(time0, time1, output_box)
        {
            return false;
        }
        *output_box = AAbb::new(
            output_box.min() + self.offset,
            output_box.max() + self.offset,
        );
        true
    }
}

pub struct Rotatey {
    ptr: Option<Arc<dyn Hiitable>>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AAbb,
}

impl Rotatey {
    pub fn new(p: Option<Arc<dyn Hiitable>>, angle: f64) -> Self {
        let mut bbox_0 = AAbb::new_0();
        let radians = degrees_to_radians(angle);
        let sin_theta_0 = radians.sin();
        let cos_theta_0 = radians.cos();
        let hashbox0 = p.clone().unwrap().bounding_box(0.0, 1.0, &mut bbox_0);

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox_0.clone().max().x()
                        + (1 - i) as f64 * bbox_0.clone().min().x();
                    let y = j as f64 * bbox_0.clone().max().y()
                        + (1 - j) as f64 * bbox_0.clone().min().y();
                    let z = k as f64 * bbox_0.clone().max().z()
                        + (1 - k) as f64 * bbox_0.clone().min().z();

                    let newx = cos_theta_0 * x + sin_theta_0 * z;
                    let newz = -sin_theta_0 * x + cos_theta_0 * z;

                    let taster = Vec3::new(newx, y, newz);

                    min.x = fmin(min.x(), taster.x());
                    min.y = fmin(min.y(), taster.y());
                    min.z = fmin(min.z(), taster.z());

                    max.x = fmax(max.x(), taster.x());
                    max.y = fmax(max.y(), taster.y());
                    max.z = fmax(max.z(), taster.z());
                }
            }
        }
        bbox_0 = AAbb::new(min, max);
        Self {
            ptr: p,
            sin_theta: sin_theta_0,
            cos_theta: cos_theta_0,
            hasbox: hashbox0,
            bbox: bbox_0,
        }
    }
}

impl Hiitable for Rotatey {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.ori();
        let mut direction = r.direc();
        origin.x = self.cos_theta * r.ori().x() - self.sin_theta * r.ori().z();
        origin.z = self.sin_theta * r.ori().x() + self.cos_theta * r.ori().z();

        direction.x = self.cos_theta * r.direc().x() - self.sin_theta * r.direc().z();
        direction.z = self.sin_theta * r.direc().x() + self.cos_theta * r.direc().z();

        let rotated_r = Ray::new(origin, direction, r.tm());

        if !self.ptr.clone().unwrap().hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.point3;
        let mut normal = rec.normal;

        p.x = self.cos_theta * rec.point3.x() + self.sin_theta * rec.point3.z();
        p.z = -self.sin_theta * rec.point3.x() + self.cos_theta * rec.point3.z();

        normal.x = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
        normal.z = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

        rec.point3 = p;
        rec.set_front_size(&rotated_r, &normal);

        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AAbb) -> bool {
        *output_box = self.bbox.clone();
        self.hasbox
    }
}

pub struct Isotropic {
    albedo: Option<Arc<dyn Texture>>,
}

impl Isotropic {
    pub fn new1(c: Vec3) -> Self {
        Self {
            albedo: Some(Arc::new(SolidColor::new(c))),
        }
    }

    pub fn new2(a: Option<Arc<dyn Texture>>) -> Self {
        Self { albedo: a }
    }
}

impl Material for Isotropic {
    fn emitted(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.point3, Vec3::random_in_unit_sphere(), r_in.tm());
        *attenuation = self
            .albedo
            .clone()
            .unwrap()
            .value(rec.u, rec.v, &rec.point3);
        true
    }
}
