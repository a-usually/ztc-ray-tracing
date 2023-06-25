mod color;
mod ray;
mod vec3;
mod rtweekend;
mod hittable_list;
mod hiitable;
mod object;

use color::{color, write_color};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use object::HitRecord;
pub use ray::Ray;
use std::fs::File;
pub use vec3::Vec3;
pub use rtweekend::{degrees_to_radians, random_f64, random_f64_1};
pub use hittable_list::HittableList;
pub use hiitable::Hiitable;
pub use object::Sphere;

const AUTHOR: &str = "Zhang Tongcheng";
const INFINITY: f64 = f64::INFINITY;

fn ray_color(r: &Ray, world: &mut HittableList) -> [u8; 3] {
    let mut rec: HitRecord = HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        let temp1: [u8;3] = color(1.0,1.0, 1.0);
        let mut tmp: [u8;3] = [0; 3];

        let x1 = temp1[0] as f64;
        let y1 =  temp1[1] as f64;
        let z1 = temp1[2] as f64;

        tmp[0] = (x1 * 0.5) as u8;
        tmp[1] = (y1 * 0.5) as u8;
        tmp[2] = (z1 * 0.5) as u8;

        return tmp;
    }
    else{
        let unit_direction = r.direc.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);

        let tmp1 = color(1.0, 1.0, 1.0);
        let tmp2 = color(0.5, 0.7, 1.0);
        let mut tmp: [u8; 3] = [0; 3];

        tmp[0] = (tmp1[0] as f64 * (1.0 - t) + (tmp2[0] as f64) * t) as u8;
        tmp[1] = (tmp1[1] as f64 * (1.0 - t) + (tmp2[1] as f64) * t) as u8;
        tmp[2] = (tmp1[2] as f64 * (1.0 - t) + (tmp2[2] as f64) * t) as u8;
    //println!("tmp={}{}{}", tmp[0], tmp[1], tmp[2]);
        return tmp
    }
}

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    let height = 800;
    let width = 800;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal: Vec3 = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical: Vec3 = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner: Vec3 = origin.clone()
        - horizontal.clone() / 2.0
        - vertical.clone() / 2.0
        - Vec3::new(0.0, 0.0, focal_length);

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap());

     // World
    let mut world: HittableList = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5 ,));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ));

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in (0..height).rev() {
        for i in 0..width {
            let u = (i as f64) / (width as f64 - 1.0);
            let v = (j as f64) / (height as f64 - 1.0);

            let ray_1: Ray = Ray {
                ori: origin.clone(),
                direc: lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v
                    - origin.clone(),
            };
            //ray_1.info();
            let pixel_color = ray_color(&ray_1, &mut world);
            write_color(pixel_color, &mut img, i, height - j - 1);
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