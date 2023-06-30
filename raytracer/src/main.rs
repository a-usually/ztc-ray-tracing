mod aabb;
mod aarect;
mod r#box;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hiitable;
mod hittable_list;
mod material;
mod moving_sphere;
mod object;
mod perlin;
mod ray;
mod rtweekend;
mod texture;
mod vec3;

pub use crate::aarect::{Xyrect, Xzrect, Yzrect};
pub use bvh::BvhNode;
pub use camera::Camera;
use color::write_color;
pub use constant_medium::ConstantMedium;
pub use hiitable::Hiitable;
pub use hittable_list::HittableList;
use image::{ImageBuffer};
use indicatif::ProgressBar;
pub use material::{Dielectric, DiffLight, Lambertian, Material, Metal, Rotatey, Translate};
pub use moving_sphere::MovingSphere;
use object::HitRecord;
pub use object::Sphere;
pub use perlin::Perlin;
pub use r#box::Box;
pub use ray::Ray;
pub use rtweekend::{degrees_to_radians, random_f64, random_f64_1};
use std::fs::File;
use std::thread;
use std::sync::{Arc, Mutex};
pub use texture::{CheckerTexture, ImageTexture, NoiseTexture, Texture};
pub use vec3::Vec3;

const AUTHOR: &str = "Zhang Tongcheng";
const INFINITY: f64 = f64::INFINITY;

fn ray_color(r: &Ray, background: &Vec3, world: &mut HittableList, depth: i32) -> Vec3 {
    let mut rec: HitRecord = HitRecord::new();
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return *background;
    }
    let mut scattered = Ray::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        random_f64_1(0.0, 1.0),
    );
    let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
    let emitter = rec.mat.clone().unwrap().emitted(rec.u, rec.v, &rec.point3);
    if !rec
        .mat
        .clone()
        .unwrap()
        .scatter(r, &mut rec, &mut attenuation, &mut scattered)
    {
        return emitter;
    }
    //println!("x:{}",attenuation.x());
    emitter
        + Vec3::elemul(
            &attenuation,
            &ray_color(&scattered, background, world, depth - 1),
        )
    // } else {
    //     let unit_direction = r.direc.unit();
    //     let t = 0.5 * (unit_direction.y() + 1.0);

    //     let tmp1 = Vec3::new(1.0, 1.0, 1.0);
    //     let tmp2 = Vec3::new(0.5, 0.7, 1.0);

    //     let x1 = tmp1.x * (1.0 - t) + tmp2.x * t;
    //     let y1 = tmp1.y * (1.0 - t) + tmp2.y * t;
    //     let z1 = tmp1.z * (1.0 - t) + tmp2.z * t;
    //     //println!("tmp={}{}{}", tmp[0], tmp[1], tmp[2]);
    //     Vec3::new(x1, y1, z1)
    // }
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

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Option<Arc<dyn Material>>;
                if choose_mat < 0.8 {
                    //difuse
                    let albedo = Vec3::elemul(&Vec3::random_vec3_1(), &Vec3::random_vec3_1());
                    sphere_material = Some(Arc::new(Lambertian::new1(&albedo)));
                    let center2 = center + Vec3::new(0.0, random_f64_1(0.0, 0.5), 0.0);
                    world.add(Some(Arc::new(MovingSphere::new(
                        center,
                        center2,
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

fn two_sphere() -> HittableList {
    let mut objects: HittableList = HittableList::new();
    let checker: Option<Arc<dyn Texture>> = Some(Arc::new(CheckerTexture::new_2(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    )));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Some(Arc::new(Lambertian::new2(&checker))),
    ))));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Some(Arc::new(Lambertian::new2(&checker))),
    ))));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects: HittableList = HittableList::new();
    let pertext: Option<Arc<dyn Texture>> = Some(Arc::new(NoiseTexture::new_0(4.0)));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new2(&pertext))),
    ))));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Some(Arc::new(Lambertian::new2(&pertext))),
    ))));

    objects
}

fn earth() -> HittableList {
    let earth_texture: Option<Arc<dyn Texture>> = Some(Arc::new(ImageTexture::new("earthmap.jpg")));
    let earth_surface: Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new2(&earth_texture)));
    let globe: Option<Arc<dyn Hiitable>> = Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));
    let mut world_0: HittableList = HittableList::new();
    world_0.add(globe);
    world_0
}

fn simple_silght() -> HittableList {
    let mut objects: HittableList = HittableList::new();
    let pertext: Option<Arc<dyn Texture>> = Some(Arc::new(NoiseTexture::new_0(4.0)));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Some(Arc::new(Lambertian::new2(&pertext.clone()))),
    ))));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Some(Arc::new(Lambertian::new2(&pertext.clone()))),
    ))));

    let difflight: Option<Arc<dyn Material>> =
        Some(Arc::new(DiffLight::new2(Vec3::new(4.0, 4.0, 4.0))));
    objects.add(Some(Arc::new(Xyrect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        difflight.clone(),
    ))));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    ))));
    objects
}

fn cornell_box() -> HittableList {
    let mut objects: HittableList = HittableList::new();

    let red: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.65, 0.05, 0.05))));
    let white: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.73, 0.73, 0.73))));
    let green: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.12, 0.45, 0.15))));
    let light: Option<Arc<dyn Material>> =
        Some(Arc::new(DiffLight::new2(Vec3::new(15.0, 15.0, 15.0))));

    objects.add(Some(Arc::new(Yzrect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    ))));
    objects.add(Some(Arc::new(Yzrect::new(
        0.0, 555.0, 0.0, 555.0, 0.0, red,
    ))));
    objects.add(Some(Arc::new(Xzrect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    ))));
    objects.add(Some(Arc::new(Xzrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    ))));
    objects.add(Some(Arc::new(Xzrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ))));
    objects.add(Some(Arc::new(Xyrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ))));
    objects.add(Some(Arc::new(Box::new(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        white.clone(),
    ))));
    objects.add(Some(Arc::new(Box::new(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        white.clone(),
    ))));

    let mut box1: Option<Arc<dyn Hiitable>> = Some(Arc::new(Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    )));
    box1 = Some(Arc::new(Rotatey::new(box1, 15.0)));
    box1 = Some(Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0))));
    objects.add(box1);

    let mut box2: Option<Arc<dyn Hiitable>> = Some(Arc::new(Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    )));
    box2 = Some(Arc::new(Rotatey::new(box2, -18.0)));
    box2 = Some(Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0))));
    objects.add(box2);

    objects
}

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.65, 0.05, 0.05))));
    let white: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.73, 0.73, 0.73))));
    let green: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.12, 0.45, 0.15))));
    let light: Option<Arc<dyn Material>> =
        Some(Arc::new(DiffLight::new2(Vec3::new(7.0, 7.0, 7.0))));
    objects.add(Some(Arc::new(Yzrect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    ))));
    objects.add(Some(Arc::new(Yzrect::new(
        0.0, 555.0, 0.0, 555.0, 0.0, red,
    ))));
    objects.add(Some(Arc::new(Xzrect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    ))));
    objects.add(Some(Arc::new(Xzrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ))));
    objects.add(Some(Arc::new(Xzrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    ))));
    objects.add(Some(Arc::new(Xyrect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ))));

    let mut box1: Option<Arc<dyn Hiitable>> = Some(Arc::new(Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    )));
    box1 = Some(Arc::new(Rotatey::new(box1, 15.0)));
    box1 = Some(Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0))));
    objects.add(Some(Arc::new(ConstantMedium::new2(
        box1,
        0.01,
        Vec3::new(0.0, 0.0, 0.0),
    ))));

    let mut box2: Option<Arc<dyn Hiitable>> = Some(Arc::new(Box::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white.clone(),
    )));
    box2 = Some(Arc::new(Rotatey::new(box2, -18.0)));
    box2 = Some(Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0))));
    objects.add(Some(Arc::new(ConstantMedium::new2(
        box2,
        0.01,
        Vec3::new(1.0, 1.0, 1.0),
    ))));

    objects
}

fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.48, 0.83, 0.53))));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64_1(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Some(Arc::new(Box::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            ))));
        }
    }
    let mut objects: HittableList = HittableList::new();
    objects.add(Some(Arc::new(BvhNode::new2(&mut boxes1, 0.0, 1.0))));

    let light: Option<Arc<dyn Material>> =
        Some(Arc::new(DiffLight::new2(Vec3::new(7.0, 7.0, 7.0))));
    objects.add(Some(Arc::new(Xzrect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    ))));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.7, 0.3, 0.1))));
    objects.add(Some(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    ))));

    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Some(Arc::new(Dielectric::new(1.5))),
    ))));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Some(Arc::new(Metal::new(&Vec3::new(0.8, 0.8, 0.9), 1.0))),
    ))));

    let mut boundary: Option<Arc<dyn Hiitable>> = Some(Arc::new(Sphere::new(
        &Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Some(Arc::new(Dielectric::new(1.5))),
    )));
    objects.add(boundary.clone());
    objects.add(Some(Arc::new(ConstantMedium::new2(
        boundary.clone(),
        0.2,
        Vec3::new(0.2, 0.4, 0.9),
    ))));
    boundary = Some(Arc::new(Sphere::new(
        &Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Some(Arc::new(Dielectric::new(1.5))),
    )));
    objects.add(Some(Arc::new(ConstantMedium::new2(
        boundary.clone(),
        0.0001,
        Vec3::new(1.0, 1.0, 1.0),
    ))));

    let emat: Option<Arc<dyn Material>> = Some(Arc::new(Lambertian::new2(&Some(Arc::new(
        ImageTexture::new("earthmap.jpg"),
    )))));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    ))));
    let pertext: Option<Arc<dyn Texture>> = Some(Arc::new(NoiseTexture::new_0(0.1)));
    objects.add(Some(Arc::new(Sphere::new(
        &Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Some(Arc::new(Lambertian::new2(&pertext))),
    ))));

    let mut boxes2 = HittableList::new();
    let white: Option<Arc<dyn Material>> =
        Some(Arc::new(Lambertian::new1(&Vec3::new(0.73, 0.73, 0.73))));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Some(Arc::new(Sphere::new(
            &Vec3::random_vec3_2(0.0, 165.0),
            10.0,
            white.clone(),
        ))));
    }

    objects.add(Some(Arc::new(Translate::new(
        Some(Arc::new(Rotatey::new(
            Some(Arc::new(BvhNode::new2(&mut boxes2, 0.0, 1.0))),
            15.0,
        ))),
        Vec3::new(-100.0, 270.0, 395.0),
    ))));

    objects
}

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    // get environment variable CI, which is true for GitHub Actions
    let is_ci = is_ci();

    println!("CI: {}", is_ci);

    let aspect_ratio = 1.0;
    let height = 900;
    let width = 900;
    let path = "output/test.jpg";
    let quality = 60; // From 0 to 100, suggested value: 60
    let samples_per_pixel = 10;
    let max_depth = 50;

    // Create image data
    let img = Arc::new(Mutex::new(ImageBuffer::new(width.try_into().unwrap(), height.try_into().unwrap())));

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

    let world;

    // Progress bar UI powered by library `indicatif`
    // You can use indicatif::ProgressStyle to make it more beautiful
    // You can also use indicatif::MultiProgress in multi-threading to show progress of each thread
    let bar = if is_ci {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    //camera
    let dist_to_focus = 10.0;

    let vfov: f64;
    let mut aperture = 0.0;

    let lookfrom: Vec3;
    let lookat: Vec3;
    let mut vup = Vec3::new(0.0, 1.0, 0.0);
    let background: Vec3;
    let time_start = 0.0;
    let time_end = 1.0;

    match 0 {
        1 => {
            world = random_scene();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vup = Vec3::new(0.0, 1.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = two_sphere();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vup = Vec3::new(0.0, 1.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            world = two_perlin_spheres();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        4 => {
            world = earth();
            background = Vec3::new(0.7, 0.8, 1.0);
            lookfrom = Vec3::new(13.0, 2.0, 3.0);
            lookat = Vec3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        5 => {
            world = simple_silght();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(26.0, 3.0, 6.0);
            lookat = Vec3::new(0.0, 2.0, 0.0);
            vfov = 20.0;
        }
        6 => {
            world = cornell_box();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(278.0, 278.0, -800.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            background = Vec3::new(0.0, 0.0, 0.0);
            lookfrom = Vec3::new(478.0, 278.0, -600.0);
            lookat = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        // _ => {
        //     lookfrom = Vec3::new(0.0, 0.0, 0.0);
        //     lookat = Vec3::new(0.0, 0.0, 0.0);
        //     vfov = 20.0;
        //     background = Vec3::new(0.0, 0.0, 0.0);
        // }
    }

    let cam: Camera = Camera::new(
        aspect_ratio,
        &lookfrom,
        &lookat,
        &vup,
        vfov,
        aperture,
        (dist_to_focus, time_start, time_end),
    );

    let job_times = 10;
    let mut handles = vec![];
    //image

    for c in 0..job_times {
        let mut world_0 = world.clone();
        let bar_0 = bar.clone();
        let img_0 = img.clone();
        let cam_0 = cam.clone();
        let handle = thread::spawn(move || {
            let height_start = height * c / job_times;
            let height_end = height * (c + 1) / job_times;
            for j in height_start..height_end {
                for i in 0..width {
                    let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixel {
                        let u = ((i as f64) + random_f64()) / (width as f64 - 1.0);
                        let v = ((j as f64) + random_f64()) / (height as f64 - 1.0);
                        let r = cam_0.get_ray(u, v);
                        let tmp = ray_color(&r, &background, &mut world_0, max_depth); //[0-1]
                        pixel_color.x += tmp.x;
                        pixel_color.y += tmp.y;
                        pixel_color.z += tmp.z;
        
                        //ray_1.info();
                    }
                    //pixel_color.info();
                    write_color(&pixel_color, &mut img_0.lock().unwrap(), i, height - j - 1, samples_per_pixel); //[0-255*sample]
                    bar_0.inc(1);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Finish progress bar
    bar.finish();

    // Output image to file
    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image = image::DynamicImage::ImageRgb8(Mutex::into_inner(Arc::into_inner(img).unwrap()).unwrap());
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
