mod aabb;
mod bvh;
mod camera;
mod color;
mod hiitable;
mod hittable_list;
mod material;
mod moving_sphere;
mod object;
mod ray;
mod rtweekend;
mod texture;
mod vec3;

pub use camera::Camera;
use color::write_color;
pub use hiitable::Hiitable;
pub use hittable_list::HittableList;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
pub use material::{Lambertian, Material, Metal};
pub use moving_sphere::MovingSphere;
use object::HitRecord;
pub use object::Sphere;
pub use ray::Ray;
pub use rtweekend::{degrees_to_radians, random_f64, random_f64_1};
pub use texture::{CheckerTexture, Texture};
pub use vec3::Vec3;

use std::fs::File;
use std::sync::Arc;

use crate::material::Dielectric;

const AUTHOR: &str = "Zhang Tongcheng";
const INFINITY: f64 = f64::INFINITY;

fn ray_color(r: &Ray, world: &mut HittableList, depth: i32) -> Vec3 {
    let mut rec: HitRecord = HitRecord::new();
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            random_f64_1(0.0, 1.0),
        );
        let mut attenuation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        if rec
            .mat
            .clone()
            .unwrap()
            .scatter(r, &mut rec, &mut attenuation, &mut scattered)
        {
            return Vec3::elemul(&attenuation, &ray_color(&scattered, world, depth - 1));
        }
        Vec3::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction = r.direc.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);

        let tmp1 = Vec3::new(1.0, 1.0, 1.0);
        let tmp2 = Vec3::new(0.5, 0.7, 1.0);

        let x1 = tmp1.x * (1.0 - t) + tmp2.x * t;
        let y1 = tmp1.y * (1.0 - t) + tmp2.y * t;
        let z1 = tmp1.z * (1.0 - t) + tmp2.z * t;
        //println!("tmp={}{}{}", tmp[0], tmp[1], tmp[2]);
        Vec3::new(x1, y1, z1)
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker: Option<Arc<dyn Texture>> = Some(Arc::new(CheckerTexture::new_2(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    )));
    world.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new2(&checker))),
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Option<Arc<dyn Material>>;
                if choose_mat < 0.8 {
                    //difuse
                    let albedo = Vec3::elemul(&Vec3::random_vec3_1(), &Vec3::random_vec3_1());
                    sphere_material = Some(Arc::new(Lambertian::new1(&albedo)));
                    let center2 = center.clone() + Vec3::new(0.0, random_f64_1(0.0, 0.5), 0.0);
                    world.add(Some(Arc::new(MovingSphere::new(
                        center.clone(),
                        center2.clone(),
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    ))));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::random_vec3_2(0.5, 1.0);
                    let fuzz = random_f64_1(0.0, 0.5);
                    sphere_material = Some(Arc::new(Metal::new(&albedo, fuzz)));
                    world.add(Some(Arc::new(Sphere::new(&center, 0.2, sphere_material))));
                } else {
                    //glass
                    sphere_material = Some(Arc::new(Dielectric::new(1.5)));
                    world.add(Some(Arc::new(Sphere::new(&center, 0.2, sphere_material))));
                }
            }
        }
    }

    let material1: Option<Arc<dyn Material>> = Some(Arc::new(Dielectric::new(1.5)));
    world.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    ))));

    let material2: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.4, 0.2, 0.1))));
    world.add(Some(Arc::new(Sphere::new(
        &Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    ))));

    let material3: Option<Arc<dyn Material>> =
        Some(Arc::new(Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Some(Arc::new(Sphere::new(
        &Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    ))));

    world
}

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    let aspect_ratio = 16.0 / 9.0;
    let height = 225;
    let width = 400;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let samples_per_pixel = 4;
    let max_depth = 20;

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

    // World
    // let mut world: HittableList = HittableList::new();

    // let material_ground: Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(&Vec3::new(0.8,0.8,0.0))));
    // let material_center: Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new(&Vec3::new(0.1,0.2,0.5))));
    // let material_left: Option<Arc<dyn Material>> = Some(Arc::new(Dielectric::new(1.5)));
    // let material_right: Option<Arc<dyn Material>> = Some(Arc::new(Metal::new(&Vec3::new(0.8,0.6,0.2), 0.0 )));

    // world.add(Some(Arc::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0),100.0,material_ground))));
    // world.add(Some(Arc::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0),0.5,material_center))));
    // world.add(Some(Arc::new(Sphere::new(&Vec3::new(-1.0, 0.0, -1.0),0.5,material_left))));
    // world.add(Some(Arc::new(Sphere::new(&Vec3::new(1.0, 0.0, -1.0),0.5,material_right))));

    let mut world = random_scene();

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    //camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let time_start = 0.0;
    let time_end = 1.0;
    let cam: Camera = Camera::new(
        aspect_ratio,
        &lookfrom,
        &lookat,
        &vup,
        vfov,
        aperture,
        dist_to_focus,
        time_start,
        time_end,
    );

    //image
    for j in 0..height {
        for i in 0..width {
            let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = ((i as f64) + random_f64()) / (width as f64 - 1.0);
                let v = ((j as f64) + random_f64()) / (height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                let tmp = ray_color(&r, &mut world, max_depth); //[0-1]
                pixel_color.x += tmp.x;
                pixel_color.y += tmp.y;
                pixel_color.z += tmp.z;

                //ray_1.info();
            }
            //pixel_color.info();
            write_color(&pixel_color, &mut img, i, height - j - 1, samples_per_pixel); //[0-255*sample]
            bar.inc(1);
        }
    }

    // Finish progress bar
    bar.finish();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
