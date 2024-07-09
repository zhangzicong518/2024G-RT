use crate::color::*;
use crate::vec3::*;
use crate::utils::*;
use crate::hitable::*;
use crate::ray::*;

use rand::prelude::*;
use indicatif::ProgressBar;
use std::fs::File;
use image::{ImageBuffer, RgbImage}; 

#[derive(Copy, Clone)]
pub struct Camera {
    camera_center: Vec3,
    aspect_ratio: f64,
    width: u32,
    height: u32,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_zero_loc: Vec3, // upper_left point
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

        Camera {
           camera_center,
           aspect_ratio,
           width,
           height,
           pixel_delta_u,
           pixel_delta_v,
           pixel_zero_loc,
        }
    }

    pub fn ray_color(r: &Ray, world: &hittable_list) -> Vec3 {
        if let Some(rec) = world.hit(r,0.0,core::f64::INFINITY) {
            return (rec.normal + Vec3::new(1.0,1.0,1.0)) * 0.5;
        }
        let unit_direction = unit_vec(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        let pixel = Vec3::new(1.0, 1.0, 1.0) * (1.0 -a) + Vec3::new(0.5, 0.7, 1.0) * a;
        pixel
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
                let pixel_center = self.pixel_zero_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.camera_center;
                let r = Ray::new(self.camera_center, ray_direction, 0.0);
                let pixel = Self::ray_color(&r,&world);
                pixel_color = color(pixel.x(), pixel.y(), pixel.z());
                write_color(pixel_color, img, i as usize, j as usize); // mutable ref no need to be declared twice
                bar.inc(1);
            }
        }
        bar.finish();
    }
    
}