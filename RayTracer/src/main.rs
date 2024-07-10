use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;

mod vec3;
mod color;
mod utils;
mod ray;
mod hitable;
mod camera;
mod interval;
mod material;

pub use crate::vec3::*;
pub use crate::color::*;
pub use crate::utils::*;
pub use crate::ray::*;
pub use crate::hitable::*;
pub use crate::camera::*;
pub use crate::interval::*;
pub use crate::material::*;

use std::rc::Rc;


const AUTHOR: &str = "ZhangZicong";

fn main() {
    let path = "output/hollow_glass_sphere.jpg";
    let width = 800;
    let height = 450;
    let quality = 60;

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1.0/1.5));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    let mut spheres = vec![
        Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground
        ),
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.2),
            0.5,
            material_center
        ),
        Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left
        ),
        Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.4,
            material_bubble
        ),
        Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            material_right
        )
    ];

    let world = hittable_list::new(spheres);
    
    let camera = Camera::new(width,height);

    camera.render(&world,&mut img);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
