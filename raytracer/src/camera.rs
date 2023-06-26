use crate::{ray, rtweekend, vec3};

pub use ray::Ray;
pub use rtweekend::degrees_to_radians;
pub use vec3::Vec3;

pub struct Camera {
    // aspect_ratio: f64,
    // vfov: f64,
    // aperture: f64,
    // focus_dist: f64,
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        lookfrom: &Vec3,
        lookat: &Vec3,
        vup: &Vec3,
        vfov: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w_0 = (lookfrom.clone() - lookat.clone()).unit();
        let u_0 = Vec3::cross(vup, &w_0).unit();
        let v_0 = Vec3::cross(&w_0, &u_0);

        Self {
            // aspect_ratio: 0.0,
            // vfov: 0.0,
            // aperture: 0.0,
            // focus_dist: 0.0,
            origin: lookfrom.clone(),
            horizontal: u_0.clone() * viewport_width * focus_dist,
            vertical: v_0.clone() * viewport_height * focus_dist,
            lower_left_corner: lookfrom.clone()
                - u_0.clone() * viewport_width * focus_dist / 2.0
                - v_0.clone() * viewport_height * focus_dist / 2.0
                - w_0.clone() * focus_dist,
            w: w_0,
            u: u_0,
            v: v_0,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offest = self.u.clone() * rd.x + self.v.clone() * rd.y;

        Ray::new(
            self.origin.clone() + offest.clone(),
            self.lower_left_corner.clone()
                + self.horizontal.clone() * s
                + self.vertical.clone() * t
                - self.origin.clone()
                - offest,
        )
    }
}
