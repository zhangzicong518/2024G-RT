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
use std::f64::consts::PI;

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
    pub vfov: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        width: u32,
        height: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64, 
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {

        let camera_center = look_from;
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let aspect_ratio = (width as f64/ height as f64);
        let viewport_width = viewport_height * aspect_ratio;
        
        let w = unit_vec(look_from - look_at);
        let u = unit_vec(vup.cross(w));
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = v * -viewport_height;
        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height as f64;
        let viewport_upper_left = camera_center - (w * focus_dist) - (viewport_u / 2.0) -(viewport_v / 2.0);
        let pixel_zero_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let defocus_radius = focus_dist * ((defocus_angle / 2.0) as f64).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
           vfov,
           look_from,
           look_at,
           vup,
           defocus_angle,
           focus_dist,
           defocus_disk_u,
           defocus_disk_v,
        }
    }

    pub fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        self.camera_center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }

    pub fn ray_color(r: &Ray, world: &hittable_list, depth: u32) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(r, &Interval::new(0.001,core::f64::INFINITY)) {
            if let Some((attenuation, new_ray)) = rec.material.scatter(r, &rec) {
                let color = Self::ray_color(&new_ray, world, depth - 1); 
                return  Vec3::new(color.x * attenuation.x, color.y * attenuation.y, color.z * attenuation.z);
            }
            else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }
        let unit_direction = unit_vec(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        let pixel = Vec3::new(1.0, 1.0, 1.0) * (1.0 -a) + Vec3::new(0.5, 0.7, 1.0) * a;
        pixel
    }
    
    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = random_squre();
        let pixel_center = self.pixel_zero_loc + (self.pixel_delta_u * (i as f64 + offset.x)) + (self.pixel_delta_v * (j as f64 + offset.y));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        }
        else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_center - ray_origin;
        Ray::new(ray_origin, ray_direction, 0.0)
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