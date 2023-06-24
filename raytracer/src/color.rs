use image::RgbImage;
/// the multi-sample write_color() function
pub fn write_color(pixel_color: [u8; 3], img: &mut RgbImage, i: usize, j: usize) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb(pixel_color);
    // Write the translated [0,255] value of each color component.
}

pub fn color(x: f64, y: f64, z: f64) -> [u8; 3] {
    let mut tmp: [u8; 3] = [0, 0, 0]; //[0;3]
    tmp[0] = (x * 255.0) as u8;
    tmp[1] = (y * 255.0) as u8;
    tmp[2] = (z * 255.0) as u8;
    tmp
}
