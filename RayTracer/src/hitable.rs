//use crate::{material::Material, ray::Ray, vec3::Vec3};
use crate::{ray::Ray, vec3::Vec3};
use crate::interval::*;
use crate::material::*;

use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material + Send + Sync>,
    pub front_face: bool,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl HitRecord {
    pub fn new(t: f64, point: Vec3, normal: Vec3, material: Arc<dyn Material + Send + Sync>, front_face: bool) -> HitRecord {
        HitRecord {
            t,
            point,
            normal,
            material,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = (r.direction() * outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        }
        else {
            self.normal = outward_normal * (-1.0);
        }
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let vec = self.center - r.origin();
        let a = r.direction().squared_length();
        let h = (r.direction() * vec);
        let c = vec.squared_length() - self.radius *self.radius;
        let delta = h * h - a * c;

        if delta > 0.0 {
            let mut temp = (h-delta.sqrt()) / a;
            if ray_t.surrounds(temp) {
                let hit_point = r.at(temp);
                let front_face = (hit_point - self.center) * r.direction() < 0.0;
                let normal = if front_face {
                    (hit_point - self.center) * (1.0 / self.radius)
                } 
                else {
                    (self.center - hit_point) * (1.0 / self.radius)
                };
                return Some(HitRecord {
                    t: temp,
                    point: hit_point,
                    normal: normal,
                    material: Arc::clone(&self.material),
                    front_face,
                });
            }
        }
        None
    }
}

pub struct hittable_list {
    spheres: Vec<Sphere>,
}

impl hittable_list {
    pub fn new(spheres: Vec<Sphere>) -> hittable_list {
        hittable_list { spheres }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.tmax;
        let mut maybe_hit: Option<HitRecord> = None;
        for sphere in self.spheres.iter() {
            if let Some(hit) = sphere.hit(&ray, ray_t) {
                closest_so_far = if hit.t < closest_so_far {
                    let new_pos = hit.t;
                    maybe_hit = Some(hit);
                    new_pos
                } else {
                    closest_so_far
                };
            }
        }
        maybe_hit
    }
}