use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;
use std::rc::Rc;
use std::f64::consts::PI;

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


const AUTHOR: &str = "ZhangZicong";

fn main() {
    let path = "output/Final_scene.jpg";
    let width = 1200;
    let height = 675;
    let quality = 60;
    let Rad = (PI / 4.0).cos();

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let mut spheres: Vec<Sphere> = Vec::new();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64_0_1();
            let center = Vec3::new(a as f64 + random_f64_0_1() * 0.9, 0.2, b as f64 + random_f64_0_1() * 0.9);
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Vec3::new(random_f64_0_1() * random_f64_0_1(), 
                                           random_f64_0_1() * random_f64_0_1(),  
                                           random_f64_0_1() * random_f64_0_1());
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    spheres.push(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::new(random_f64_range(0.5, 1.0), random_f64_range(0.5, 1.0), random_f64_range(0.5, 1.0));
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    spheres.push(Sphere::new(center, 0.2, sphere_material));
                } else {
                    //glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    spheres.push(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    spheres.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    let material1 = Rc::new(Dielectric::new(1.5));
    spheres.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    spheres.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    spheres.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let world = hittable_list::new(spheres);
    
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let vfov: f64 = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let samples_per_pixel = 500;
    let max_depth = 50;

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist);
    
    camera.render(&world,&mut img);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
