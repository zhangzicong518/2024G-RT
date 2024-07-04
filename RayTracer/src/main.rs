use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;

mod vec3;
mod color;
mod utils;
mod ray;

pub use crate::vec3::*;
pub use crate::color::*;
pub use crate::utils::*;
pub use crate::ray::*;

const AUTHOR: &str = "ZhangZicong";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

pub fn ray_color(r: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, &r);
    if t > 0.0 {
        let mut n = r.at(t) - Vec3::new(0.0, 0.0, -1.0);
        n =  unit_vec(n);
        return (n + Vec3::new(1.0,1.0,1.0)) * 0.5;
    }
    let unit_direction = unit_vec(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    //println!("height: {}, a: {}",j,a);
    let pixel = Vec3::new(1.0, 1.0, 1.0) * (1.0 -a) + Vec3::new(0.5, 0.7, 1.0) * a;
    pixel
}

fn main() {
    let path = "output/sky_with_circle.jpg";
    let width = 800;
    let height = 450;
    let quality = 60;
    let bar: ProgressBar = if is_ci() {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64/ height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / width as f64;
    let pixel_delta_v = viewport_v / height as f64;
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) -(viewport_v / 2.0);
    let pixel_zero_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
    //println!("viewport_u:{}, {}, {}",viewport_u.x(),viewport_u.y(),viewport_u.z());
    //println!("viewport_v:{}, {}, {}",viewport_v.x(),viewport_v.y(),viewport_v.z());
    //println!("pixel_delta_u:{}, pixel_delta_v:{}",pixel_delta_u,pixel_delta_v);
    //println!("viewport_upper_left: {}, {}",viewport_upper_left.x(),viewport_upper_left.y());
    //let viewport_down_right = pixel_zero_loc + (pixel_delta_u * 800 as f64) + (pixel_delta_v * 450 as f64);
    //println!("viewport_donw_right: {}, {}",viewport_down_right.x(),viewport_down_right.y());


    
    let mut pixel_color = [0u8; 3];

    for j in 0..height {
        //println!("remaining lines :{}",(height-j));
        for i in 0..width {
            let pixel_center = pixel_zero_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction, 0.0);
            let pixel = ray_color(&r);
            pixel_color = color(pixel.x(), pixel.y(), pixel.z());
            write_color(pixel_color, &mut img, i as usize, j as usize);
            bar.inc(1);
        }
    }
    bar.finish();

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
