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
mod quad;
mod bvh;

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
pub use crate::quad::*;
pub use crate::bvh::*;

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
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
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
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);

    let img = camera.render(&(world.to_bvh()));
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
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
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
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
    img
}

pub fn quads() -> RgbImage {
    println!("choose quads");
    let width = 400;
    let height = 400;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let left_red = Lambertian::new_from_color(Vec3::new(1.0, 0.2, 0.2)).instancing();
    let back_green = Lambertian::new_from_color(Vec3::new(0.2, 1.0, 0.2)).instancing();
    let right_blue = Lambertian::new_from_color(Vec3::new(0.2, 0.2, 1.0)).instancing();
    let upper_orange = Lambertian::new_from_color(Vec3::new(1.0, 0.5, 0.0)).instancing();
    let lower_teal = Lambertian::new_from_color(Vec3::new(0.2, 0.8, 0.8)).instancing();

    world.add(Quad::new(
            Vec3::new(-3.0, -2.0, 5.0),
            Vec3::new(0.0, 0.0, -4.0),
            Vec3::new(0.0, 4.0, 0.0),
            left_red,
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(-2.0, -2.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
            back_green,
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(3.0, -2.0, 1.0),
            Vec3::new(0.0, 0.0, 4.0),
            Vec3::new(0.0, 4.0, 0.0),
            right_blue,
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(-2.0, 3.0, 1.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 4.0),
            upper_orange,
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(-2.0, -3.0, 5.0),
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -4.0),
            lower_teal,
        ).instancing()
    );

     
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 80.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(0.0, 0.0, 9.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
    img
}

pub fn simple_light() -> RgbImage {
    println!("choose simple light");
    let width = 400;
    let height = 225;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let pretext = NoiseTexture::new(4.0).instancing();
    let material_shadow = Lambertian::new(pretext).instancing();

    world.add(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_shadow.clone(),
        ).instancing()
    );
    world.add(Sphere::new(
            Vec3::new(0.0, 2.0, 0.0),
            2.0,
            material_shadow.clone(),
        ).instancing()
    );
    
    let material_difflight = Diffuselight::new_from_color(Vec3::new(4.0, 4.0, 4.0)).instancing();
    world.add(Quad::new(
            Vec3::new(3.0, 1.0, -2.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            material_difflight.clone(),
        ).instancing()
    );
    world.add(Sphere::new(
            Vec3::new(0.0, 7.0, 0.0),
            2.0,
            material_difflight.clone(),
        ).instancing()
    );

    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(26.0, 3.0, 6.0);
    let look_at = Vec3::new(0.0, 2.0, 0.0);
    let samples_per_pixel = 100;
    let max_depth = 50;
    let background = Vec3::new(0.0, 0.0 , 0.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
    img
}

pub fn cornell_box() -> RgbImage {
    println!("choose cornell box");
    let width = 600;
    let height = 600;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let red = Lambertian::new_from_color(Vec3::new(0.65, 0.05, 0.05)).instancing();
    let white = Lambertian::new_from_color(Vec3::new(0.73, 0.73, 0.73)).instancing();
    let green = Lambertian::new_from_color(Vec3::new(0.12, 0.45, 0.15)).instancing();
    let light = Diffuselight::new_from_color(Vec3::new(15.0, 15.0, 15.0)).instancing();

    world.add(Quad::new(
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            green.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            red.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(343.0, 554.0, 332.0),
            Vec3::new(-130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -105.0),
            light.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            white.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(555.0, 555.0, 555.0),
            Vec3::new(-555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -555.0),
            white.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(0.0, 0.0, 555.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            white.clone(),
        ).instancing()
    );

    let box1 = create_box(
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(165.0, 330.0, 165.0), 
        white.clone()
    ).instancing();
    let box1 = RotateY::new(box1, 15.0).instancing();
    let box1 = Translate::new(box1, Vec3::new(265.0, 1.0, 295.0)).instancing();
    world.add(box1);

    let box2 = create_box(
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(165.0, 165.0, 165.0), 
        white.clone()
    ).instancing();
    let box2 = RotateY::new(box2, -18.0).instancing();
    let box2 = Translate::new(box2, Vec3::new(130.0, 1.0, 65.0)).instancing();
    world.add(box2);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 40.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let samples_per_pixel = 200;
    let max_depth = 50;
    let background = Vec3::new(0.0, 0.0 , 0.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
    img
}

pub fn cornell_smoke() -> RgbImage {
    println!("choose cornell smoke");
    let width = 600;
    let height = 600;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let red = Lambertian::new_from_color(Vec3::new(0.65, 0.05, 0.05)).instancing();
    let white = Lambertian::new_from_color(Vec3::new(0.73, 0.73, 0.73)).instancing();
    let green = Lambertian::new_from_color(Vec3::new(0.12, 0.45, 0.15)).instancing();
    let light = Diffuselight::new_from_color(Vec3::new(7.0, 7.0, 7.0)).instancing();

    world.add(Quad::new(
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            green.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            red.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(113.0, 554.0, 127.0),
            Vec3::new(330.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 305.0),
            light.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            white.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(555.0, 555.0, 555.0),
            Vec3::new(-555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -555.0),
            white.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(0.0, 0.0, 555.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            white.clone(),
        ).instancing()
    );

    let box1 = create_box(
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(165.0, 330.0, 165.0), 
        white.clone()
    ).instancing();
    let box1 = RotateY::new(box1, 15.0).instancing();
    let box1 = Translate::new(box1, Vec3::new(265.0, 1.0, 295.0)).instancing();
    let box1 = ConstantMedium::new_from_color(box1, 0.01, Vec3::zero()).instancing();
    world.add(box1);

    let box2 = create_box(
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(165.0, 165.0, 165.0), 
        white.clone()
    ).instancing();
    let box2 = RotateY::new(box2, -18.0).instancing();
    let box2 = Translate::new(box2, Vec3::new(130.0, 1.0, 65.0)).instancing();
    let box2 = ConstantMedium::new_from_color(box2, 0.01, Vec3::ones()).instancing();
    world.add(box2);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 40.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let samples_per_pixel = 200;
    let max_depth = 50;
    let background = Vec3::new(0.0, 0.0 , 0.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
    img
}

pub fn final_scene(width: i32, samples_per_pixel: i32, max_depth: i32) -> RgbImage {
    println!("choose final scene");
    let height = width;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let ground = Lambertian::new_from_color(Vec3::new(0.48, 0.83, 0.53)).instancing();
    let mut boxes1 = Hittable_list::default();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let p0 = Vec3::new(
                -1000.0 + i as f64 * w,
                0.0,
                -1000.0 + j as f64 * w
            );
            let p1 = Vec3::new(
                p0.x + w,
                random_f64_range(1.0, 101.0),
                p0.z + w
            );

            boxes1.add(create_box(p0, p1, ground.clone()).instancing());
        }
    }
    world.add(boxes1.to_bvh());

    let light = Diffuselight::new_from_color(Vec3::new(7.0, 7.0, 7.0)).instancing();
    world.add(Quad::new(
            Vec3::new(123.0, 554.0, 147.0),
            Vec3::new(300.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 265.0),
            light
        ).instancing()
    ); 

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Lambertian::new_from_color(Vec3::new(0.7, 0.3, 0.1)).instancing();
    world.add(Sphere::new_moving(
            center1, 
            center2,
            50.0, 
            sphere_material
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(260.0, 150.0, 45.0),
            50.0,
            Dielectric::new(1.5).instancing()
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(0.0, 150.0, 145.0),
            50.0,
            Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0).instancing()
        ).instancing()
    );

    let boundary = Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.5).instancing()
    ).instancing();
    world.add(boundary.clone());

    world.add(ConstantMedium::new_from_color(
            boundary.clone(),
            0.2, 
            Vec3::new(0.2, 0.4, 0.9)
        ).instancing()
    );

    let boundary = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        5000.0,
        Dielectric::new(1.5).instancing()
    ).instancing();
    world.add(ConstantMedium::new_from_color(
            boundary.clone(),
            0.0001, 
            Vec3::new(1.0, 1.0, 1.0)
        ).instancing()
    );

    let path = std::env::current_dir()
        .unwrap()
        .join(Path::new("earth_map.jpg"));
    let earth_texture = ImageTexture::new(&path).instancing();
    let material_earth = Lambertian::new(earth_texture).instancing();

    world.add(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        material_earth,
    ).instancing());

    let pertext = NoiseTexture::new(0.2).instancing();
    world.add(Sphere::new(
            Vec3::new(220.0, 280.0, 300.0),
            80.0,
            Lambertian::new(pertext).instancing()
        ).instancing()
    );

    let mut boxes2 = Hittable_list::default();
    let white = Lambertian::new_from_color(Vec3::new(0.73, 0.73, 0.73)).instancing();
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Sphere::new(
                random_vec3_range(0.0, 165.0),
                10.0,
                white.clone()
            ).instancing()
        );
    }

    world.add(Translate::new(
        RotateY::new(
            boxes2.to_bvh(),
            15.0
        ).instancing(),
        Vec3::new(-100.0, 270.0, 395.0)
    ).instancing());

    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 40.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(478.0, 278.0, -600.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let background = Vec3::new(0.0, 0.0 , 0.0);

    let camera = Camera::new(width as u32, height as u32, samples_per_pixel as u32, max_depth as u32, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
    img
}

pub fn cornell_box_update() -> RgbImage {
    println!("choose cornell box smoke");
    let width = 600;
    let height = 600;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let red = Lambertian::new_from_color(Vec3::new(0.65, 0.05, 0.05)).instancing();
    let white = Lambertian::new_from_color(Vec3::new(0.73, 0.73, 0.73)).instancing();
    let green = Lambertian::new_from_color(Vec3::new(0.12, 0.45, 0.15)).instancing();
    let light = Diffuselight::new_from_color(Vec3::new(15.0, 15.0, 15.0)).instancing();

    world.add(Quad::new(
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            green.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            red.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(343.0, 554.0, 332.0),
            Vec3::new(-130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -105.0),
            light.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            white.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(555.0, 555.0, 555.0),
            Vec3::new(-555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -555.0),
            white.clone(),
        ).instancing()
    );
    world.add(Quad::new(
            Vec3::new(0.0, 0.0, 555.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            white.clone(),
        ).instancing()
    );

    let box1 = create_box(
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(165.0, 330.0, 165.0), 
        white.clone()
    ).instancing();
    let box1 = RotateY::new(box1, 15.0).instancing();
    let box1 = Translate::new(box1, Vec3::new(265.0, 1.0, 295.0)).instancing();
    world.add(box1);

    let box2 = create_box(
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(165.0, 165.0, 165.0), 
        white.clone()
    ).instancing();
    let box2 = RotateY::new(box2, -18.0).instancing();
    let box2 = Translate::new(box2, Vec3::new(130.0, 1.0, 65.0)).instancing();
    world.add(box2);

    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 40.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(278.0, 278.0, -800.0);
    let look_at = Vec3::new(278.0, 278.0, 0.0);
    let samples_per_pixel = 64;
    let max_depth = 50;
    let background = Vec3::new(0.0, 0.0 , 0.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
    img
}

pub fn solar_system() -> RgbImage {
    println!("choose solar system");
    let width = 1600;
    let height = 900;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();

    let galaxy_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("galaxy.jpeg"));
    let galaxy_texture = ImageTexture::new(&galaxy_path).instancing();
    let material_galaxy = Diffuselight::new(galaxy_texture).instancing();
    world.add(Quad::new(
            Vec3::new(-300.0, -200.0, -200.0),
            Vec3::new(0.0, 400.0, 0.0),
            Vec3::new(600.0, 0.0, 0.0),
            material_galaxy
        ).instancing()
    );

    let earth_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("earth_map.jpg"));
    let earth_texture = ImageTexture::new(&earth_path).instancing();
    let material_earth = Diffuselight::new(earth_texture).instancing();

    let mars_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("mars_map.jpg"));
    let mars_texture = ImageTexture::new(&mars_path).instancing();
    let material_mars = Diffuselight::new(mars_texture).instancing();

    let mercury_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("mercury_map.jpg"));
    let mercury_texture = ImageTexture::new(&mercury_path).instancing();
    let material_mercury = Diffuselight::new(mercury_texture).instancing();

    let neptune_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("neptune_map.jpg"));
    let neptune_texture = ImageTexture::new(&neptune_path).instancing();
    let material_neptune = Diffuselight::new(neptune_texture).instancing();

    let venus_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("venus_map.jpg"));
    let venus_texture = ImageTexture::new(&venus_path).instancing();
    let material_venus = Diffuselight::new(venus_texture).instancing();

    let uranus_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("uranus_map.jpg"));
    let uranus_texture = ImageTexture::new(&uranus_path).instancing();
    let material_uranus = Diffuselight::new(uranus_texture).instancing();

    let jupyter_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("jupyter_map.jpg"));
    let jupyter_texture = ImageTexture::new(&jupyter_path).instancing();
    let material_jupyter = Diffuselight::new(jupyter_texture).instancing();

    let saturn_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("saturn_map.jpg"));
    let saturn_texture = ImageTexture::new(&saturn_path).instancing();
    let material_saturn = Diffuselight::new(saturn_texture).instancing();

    let sun_path = std::env::current_dir()
        .unwrap()
        .join(Path::new("sun_map.jpg"));
    let sun_texture = ImageTexture::new(&sun_path).instancing();
    let material_sun = Diffuselight::new(sun_texture.clone()).instancing();

    world.add(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            6.0,
            material_sun,
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(-10.0, 3.0, -3.0), 
            1.6,  
            material_mercury,
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(12.0, -3.0, 2.0), 
            1.3,  
            material_venus,
        ).instancing()
    );
    
    world.add(Sphere::new(
            Vec3::new(-4.0, -3.0, 15.0), 
            1.8,  
            material_earth,
        ).instancing()
    );
    
    world.add(Sphere::new(
            Vec3::new(4.0, -4.0, 18.0), 
            1.6,  
            material_mars,
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(-15.0, -3.0, 15.0), 
            2.5,  
            material_jupyter,
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(-15.0, 25.0, -70.0), 
            6.0, 
            material_saturn,
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(35.0, 15.0, -50.0), 
            5.0, 
            material_uranus,
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(60.0, 10.0, -30.0), 
            5.0, 
            material_neptune,
        ).instancing()
    );
    

     
    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 60.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(0.0, 0.0, 30.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let samples_per_pixel = 400;
    let max_depth = 100;
    let background = Vec3::new(0.0, 0.0 , 0.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
    img
}

pub fn random_scene() -> RgbImage {

    println!("choose random scene");
    let width = 600;
    let height = 600;
    let Rad = (PI / 4.0).cos();

    let mut world = Hittable_list::default();
    
    let checker = CheckerTexture::new_from_color(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)).instancing();

    world.add(Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            -1000.0,
            Dielectric::new(1.5).instancing(),
        ).instancing()
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64_0_1();
            let center = Vec3::new(
                a as f64 + 0.5 * random_f64_0_1(),
                0.2,
                b as f64 + 0.5 * random_f64_0_1(),
            );
            if (center - Vec3::new(4.0, 0.4, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn MaterialTrait + Send + Sync>;

                if choose_mat < 0.6 {
                    // metal
                    let albedo = random_vec3_range(0.5, 1.0);
                    let sphere_material = Diffuselight::new_from_color(albedo).instancing();
                        world.add(Sphere::new(
                                center, 
                                0.2, 
                                sphere_material
                        ).instancing()
                    );
                } else {
                    if choose_mat < 0.8 {
                       // glass
                        sphere_material = Dielectric::new(1.5).instancing();
                        world.add(Sphere::new(
                                center, 
                                0.2, 
                                sphere_material
                            ).instancing()
                        );
                    } else {
                         // diffuse
                        let albedo = random_vec3_range(0.0, 1.0);
                        sphere_material = Lambertian::new_from_color(albedo).instancing();
                        let center2 = center + Vec3::new(0.0, random_f64_range(0.0, 0.5), 0.0);
                        world.add(Sphere::new_moving(
                                center,
                                center2,
                                0.2,
                                sphere_material,
                            ).instancing()
                        );
                        
                    }
                }
            }
        }
    }

    world.add(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Dielectric::new(1.5).instancing(),
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Diffuselight::new(checker.clone()).instancing(),
        ).instancing()
    );

    world.add(Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Metal::new(
                Vec3::new(0.7, 0.6, 0.5),
                0.0,
            ).instancing(),
        ).instancing()
    );

    let defocus_angle = 0.0;
    let focus_dist = 10.0;
    let vfov: f64 = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Vec3::new(26.0, 3.0, -16.0);
    let look_at = Vec3::new(0.0, -2.0, 0.0);
    let samples_per_pixel = 200;
    let max_depth = 50;
    let background = Vec3::new(0.0, 0.0 , 0.0);

    let camera = Camera::new(width, height, samples_per_pixel, max_depth, vfov, look_from, look_at, vup, defocus_angle, focus_dist,background);
    
    let img = camera.render(&(world.to_bvh()));
    img
}

fn main() {
    let path = "output/random_scene.jpg";
    let quality = 60;
    let choice = 13;

    let img = match choice {
        1 => bouncing_spheres(),
        2 => checkered_sphers(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 1000, 40),
        10 => final_scene(400, 250, 4),
        11 => cornell_box_update(),
        12 => solar_system(),
        _ => random_scene(),
    };

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
