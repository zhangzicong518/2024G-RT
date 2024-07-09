use image::RgbImage;
use crate::interval::*;
use crate::vec3::*;

pub fn linear_to_gamma(linear_component:f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    }
    else {
        0.0
    }
}

/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, img: &mut RgbImage, i: usize, j: usize) {
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    
    let interval:Interval = Interval::new(0.000,0.999);
    
    let transformed_pixel_color = [linear_to_gamma(pixel_color.x), linear_to_gamma(pixel_color.y), linear_to_gamma(pixel_color.z)];
    
    let color = [(256.0 * interval.clamp(transformed_pixel_color[0])) as u8, (256.0 * interval.clamp(transformed_pixel_color[1])) as u8, (256.0 * interval.clamp(transformed_pixel_color[2])) as u8];

    *pixel = image::Rgb(color);
}
