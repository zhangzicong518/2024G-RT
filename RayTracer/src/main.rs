use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;
use std::sync::Arc;
use std::f64::consts::PI;
use std::path::Path;

mod vec3;
mod color;
mod utils;
mod ray;
mod hitable;
mod camera;
mod interval;
mod material;
mod aabb;
mod sphere;
mod texture;
mod perlin;

pub use crate::vec3::*;
pub use crate::color::*;
pub use crate::utils::*;
pub use crate::ray::*;
pub use crate::hitable::*;
pub use crate::camera::*;
pub use crate::interval::*;
pub use crate::material::*;
pub use crate::aabb::*;
pub use crate::sphere::*;
pub use crate::texture::*;
pub use crate::perlin::*;

const AUTHOR: &str = "ZhangZicong";

pub fn bouncing_spheres() -> RgbImage {
    let width = 400;
    let height = 225;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let checker = CheckerTexture::new_from_color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)).instancing();
    let material_ground = Lambertian::new(checker).instancing();
    world.add(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_ground,
        ).instancing()
    );

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
                    let sphere_material = Lambertian::new_from_color(albedo).instancing();
                    world.add(Sphere::new_moving(center, center + Vec3::new(0.0, random_f64_range(0.0, 0.5), 0.0), 0.2, sphere_material).instancing());
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::new(random_f64_range(0.5, 1.0), random_f64_range(0.5, 1.0), random_f64_range(0.5, 1.0));
                    let fuzz = random_f64_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz).instancing();
                    world.add(Sphere::new(center, 0.2, sphere_material).instancing());
                } else {
                    //glass
                    let sphere_material = Dielectric::new(1.5).instancing();
                    world.add(Sphere::new(center, 0.2, sphere_material).instancing());
                }
            }
        }
    }

    let material_ground =Lambertian::new_from_color(Vec3::new(0.5, 0.5, 0.5)).instancing();
    world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground).instancing());

    let material1 = Dielectric::new(1.5).instancing();
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1).instancing());

    let material2 = Lambertian::new_from_color(Vec3::new(0.4, 0.2, 0.1)).instancing();
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2).instancing());

    let material3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0).instancing();
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3).instancing());
    
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let vfov: f64 = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist);
    
    let img = camera.render(&world);
    img
}

pub fn checkered_sphers() -> RgbImage {
    println!("choose checkered_sphers");
    let width = 400;
    let height = 225;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let checker = CheckerTexture::new_from_color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)).instancing();
    let material_ground = Lambertian::new(checker).instancing();

    world.add(Sphere::new(
            Vec3::new(0.0, -10.0, 0.0),
            10.0,
            material_ground.clone(),
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(0.0, 10.0, 0.0),
            10.0,
            material_ground.clone(),
        ).instancing()
    );
    
    let defocus_angle = 0.0;
    let focus_dist = 0.0;
    let vfov: f64 = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist);
    
    let img = camera.render(&world);
    img
}

pub fn earth() -> RgbImage {
    println!("choose earth");
    let width = 400;
    let height = 225;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let path = std::env::current_dir()
        .unwrap()
        .join(Path::new("earth_map.jpg"));

    let earth_texture = ImageTexture::new(&path).instancing();
    let material_earth = Lambertian::new(earth_texture).instancing();
    world.add(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0), 
            2.0, 
            material_earth,
        ).instancing()
    );

     
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(0.0, 0.0, 12.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist);
    
    let img = camera.render(&world);
    img
}

pub fn perlin_spheres() -> RgbImage {
    println!("choose perlin spheres");
    let width = 400;
    let height = 225;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let perlin_texture = NoiseTexture::new(4.0).instancing();
    let material_perlin = Lambertian::new(perlin_texture).instancing();
    world.add(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0), 
            1000.0, 
            material_perlin.clone(),
        ).instancing()
    );
    world.add(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0), 
            2.0, 
            material_perlin.clone(),
        ).instancing()
    );

     
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist);
    
    let img = camera.render(&world);
    img
}

fn main() {
    let path = "output/book2/perlin_noise_marbled.jpg";
    let quality = 60;
    let choice = 4;

    let img = match choice {
        1 => bouncing_spheres(),
        2 => checkered_sphers(),
        3 => earth(),
        4 => perlin_spheres(),
        _ => perlin_spheres(),
    };

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
