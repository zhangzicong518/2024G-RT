use crate::color::*;
use crate::vec3::*;
use crate::utils::*;
use crate::hitable::*;
use crate::ray::*;
use crate::interval::*;

use rand::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use image::{ImageBuffer, RgbImage}; 
use std::f64::consts::PI;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, Condvar};
use crossbeam::thread;

const HEIGHT_PARTITION: usize = 20; // multithreading parameters
const WIDTH_PARTITION: usize = 20;
const THREAD_LIMIT: usize = 20;

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
    pub background: Vec3,
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
        background: Vec3,
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
           background,
        }
    }


    pub fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        self.camera_center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }

    pub fn ray_color(&self, r: &Ray, world: &Arc<dyn Hittable + Send + Sync>, depth: u32) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();
        if !world.hit(r, Interval::new(0.001,core::f64::INFINITY), &mut rec) {
            return self.background;
        }
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::zero();
        let color_from_emission = rec.material.emitted(rec.u, rec.v, rec.point);
        if !rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }
        //println!("successfully scattered");

        let mut color_from_scattered = self.ray_color(&scattered, world, depth - 1); 
        color_from_scattered = Vec3::new(
            color_from_scattered.x * attenuation.x, 
            color_from_scattered.y * attenuation.y, 
            color_from_scattered.z * attenuation.z
        );
        color_from_emission + color_from_scattered
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
        Ray::new(ray_origin, ray_direction, random_f64_0_1())
    }

    pub fn is_ci() -> bool {
        option_env!("CI").unwrap_or_default() == "true"
    }

    pub fn render(&self, world: &Arc<dyn Hittable + Send + Sync>) -> RgbImage{
        let mut img: RgbImage = ImageBuffer::new(self.width, self.height);
        let img_mtx = Arc::new(Mutex::new(&mut img));

        let bar: ProgressBar = if Self::is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.height * self.width) as u64)
        };

        let camera = Arc::new(self.clone());
        let world = Arc::new(world);
        let bar_wrapper = Arc::new(&bar);

        thread::scope(move |thd_spawner|{
            let thread_count = Arc::new(AtomicUsize::new(0));
            let thread_number_controller = Arc::new(Condvar::new());
            
            let chunk_height = (self.height as usize + HEIGHT_PARTITION - 1) / HEIGHT_PARTITION;
            let chunk_width = (self.width as usize + WIDTH_PARTITION - 1) / WIDTH_PARTITION;
            
            for j in 0..HEIGHT_PARTITION {
              for i in 0..WIDTH_PARTITION {
                
                let lock_for_condv = Mutex::new(false);
                while !(thread_count.load(Ordering::SeqCst) < THREAD_LIMIT) {
                  thread_number_controller.wait(lock_for_condv.lock().unwrap()).unwrap();
                }
                
                let camera = Arc::clone(&camera);
                let img_mtx = Arc::clone(&img_mtx);
                let bar = Arc::clone(&bar_wrapper);
                let world = Arc::clone(&world);
                let thread_count = Arc::clone(&thread_count);
                let thread_number_controller = Arc::clone(&thread_number_controller);
                

                thread_count.fetch_add(1, Ordering::SeqCst);
                bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst))); 
      
                let _ = thd_spawner.spawn(move |_| {
                  camera.render_sub(&world, &img_mtx, &bar, 
                    i * chunk_width, (i + 1) * chunk_width, 
                    j * chunk_height, (j + 1) * chunk_height);
      
                  thread_count.fetch_sub(1, Ordering::SeqCst); 
                  bar.set_message(format!("|{} threads outstanding|", thread_count.load(Ordering::SeqCst)));
                  thread_number_controller.notify_one();
                });
      
              }
            }
          }).unwrap();
        
          bar.finish();
          img
    }

    pub fn render_sub(&self, world: &Arc<dyn Hittable + Send + Sync>, img_mtx: &Mutex<&mut RgbImage>, bar: &ProgressBar, x_min: usize, x_max: usize, y_min: usize, y_max: usize) {
        let x_max = x_max.min(self.width as usize);
        let y_max = y_max.min(self.height as usize);
        let x_min = x_min.max(0);
        let y_min = y_min.max(0); 

        // avoid situation x_min == x_max 
        if x_max > x_min && y_max > y_min {
            let mut buff: Vec<Vec<Vec3>> = vec![vec![Vec3::zero(); y_max - y_min]; x_max - x_min];

            for j in y_min..y_max {
                for i in x_min..x_max {
                    let mut pixel_color = Vec3::new(0.0,0.0,0.0);
                    for k in 0..self.samples_per_pixel{
                        let mut r = self.get_ray(i as u32, j as u32);
                        pixel_color += self.ray_color(&r,&world, self.max_depth);
                    }
                    buff[i - x_min][j - y_min] = pixel_color *self.pixel_samples_scale;
                }
                bar.inc((x_max - x_min) as u64);
            }
            let mut img = img_mtx.lock().unwrap();
            for j in y_min..y_max {
                for i in x_min..x_max {
                    write_color(buff[i - x_min][j - y_min], &mut img, i as usize, j as usize);
                }
            }
        }
    }
    
}