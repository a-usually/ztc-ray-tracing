pub use crate::rtweekend::clamp;
pub use crate::vec3::Vec3;
use image::RgbImage;

/// the multi-sample write_color() function
pub fn write_color(color: &Vec3, img: &mut RgbImage, i: usize, j: usize, samples_per_pixel: i32) {
    let mut r = color.x();
    let mut g = color.y();
    let mut b = color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / (samples_per_pixel as f64); //[0-255]
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let pixel_color = [
        (clamp(r, 0.0, 0.999) * 256.0) as u8, //r in [0-255] > 1 => clamp(...) = 1
        (clamp(g, 0.0, 0.999) * 256.0) as u8,
        (clamp(b, 0.0, 0.999) * 256.0) as u8,
    ];

    // for i in pixel_color{
    //     // print!("{},",i);
    // }
    // println!("color");
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb(pixel_color);
    // Write the translated [0,255] value of each color component.
}

// pub fn color(x: f64, y: f64, z: f64) -> [u8; 3] {
//     let mut tmp: [u8; 3] = [0; 3]; //[0;3]
//     tmp[0] = (x * 255.0) as u8;
//     tmp[1] = (y * 255.0) as u8;
//     tmp[2] = (z * 255.0) as u8;
//     tmp
// }
