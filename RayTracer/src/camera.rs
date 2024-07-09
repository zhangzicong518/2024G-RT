use crate::color::*;
use crate::vec3::*;
use crate::utils::*;
use crate::hitable::*;
use crate::ray::*;
use crate::interval::*;

use rand::prelude::*;
use indicatif::ProgressBar;
use std::fs::File;
use image::{ImageBuffer, RgbImage}; 

#[derive(Copy, Clone)]
pub struct Camera {
    pub camera_center: Vec3,
    pub aspect_ratio: f64,
    pub width: u32,
    pub height: u32,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel_zero_loc: Vec3, // upper_left point
    pub samples_per_pixel: u32,
    pub pixel_samples_scale: f64,
    pub max_depth: u32,
}

impl Camera {
    pub fn new(
        width:u32,
        height:u32,
    ) -> Camera {

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let aspect_ratio = (width as f64/ height as f64);
        let viewport_width = viewport_height * aspect_ratio;
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height as f64;
        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0,   focal_length) - (viewport_u / 2.0) -(viewport_v / 2.0);
        let pixel_zero_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let samples_per_pixel = 100;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let max_depth = 50;

        Camera {
           camera_center,
           aspect_ratio,
           width,
           height,
           pixel_delta_u,
           pixel_delta_v,
           pixel_zero_loc,
           samples_per_pixel,
           pixel_samples_scale,
           max_depth,
        }
    }

    pub fn ray_color(r: &Ray, world: &hittable_list, depth: u32) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(r, &Interval::new(0.001,core::f64::INFINITY)) {
            let direction = rec.normal + random_in_unit_shpere();
            return Self::ray_color(&Ray::new(rec.point, direction, 0.0), world, depth - 1) * 0.1;
        }
        let unit_direction = unit_vec(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        let pixel = Vec3::new(1.0, 1.0, 1.0) * (1.0 -a) + Vec3::new(0.5, 0.7, 1.0) * a;
        pixel
    }
    
    pub fn get_ray(self, i: u32, j: u32) -> Ray {
        let offset = random_squre();
        let pixel_center = self.pixel_zero_loc + (self.pixel_delta_u * (i as f64 + offset.x)) + (self.pixel_delta_v * (j as f64 + offset.y));
        let ray_direction = pixel_center - self.camera_center;
        Ray::new(self.camera_center, ray_direction, 0.0)
    }

    pub fn is_ci() -> bool {
        option_env!("CI").unwrap_or_default() == "true"
    }

    pub fn render(self, world: &hittable_list, img: &mut RgbImage) {
        let bar: ProgressBar = if Self::is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.height * self.width) as u64)
        };

        let mut pixel_color = [0u8; 3];
        for j in 0..self.height {
            //println!("remaining lines :{}",(height-j));
            for i in 0..self.width {
                let mut pixel_color = Vec3::new(0.0,0.0,0.0);
                for k in 0..self.samples_per_pixel{
                    let mut r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r,&world, self.max_depth);
                }
                write_color(pixel_color * self.pixel_samples_scale, img, i as usize, j as usize); // mutable ref no need to be declared twice
                bar.inc(1);
            }
        }
        bar.finish();
    }
    
}