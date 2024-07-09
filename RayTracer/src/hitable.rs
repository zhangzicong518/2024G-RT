//use crate::{material::Material, ray::Ray, vec3::Vec3};
use crate::{ray::Ray, vec3::Vec3};

/*
#[derive(Copy, Clone)]
pub struct HitRecord<'obj> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    //pub material: &'obj Material,
}
*/

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    //pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
            //material,
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let vec = self.center - r.origin();
        let a = r.direction().squared_length();
        let h = (r.direction() * vec);
        let c = vec.squared_length() - self.radius *self.radius;
        let delta = h * h - a * c;

        if delta > 0.0 {
            let mut temp = (h-delta.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = r.at(temp);
                return Some(HitRecord {
                    t: temp,
                    point: hit_point,
                    normal: (hit_point - self.center) * (1.0 / self.radius),
                    //material: &self.material,
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

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut maybe_hit: Option<HitRecord> = None;
        for sphere in self.spheres.iter() {
            if let Some(hit) = sphere.hit(&ray, t_min, t_max) {
                closest_so_far = if hit.t < closest_so_far {
                    maybe_hit = Some(hit);
                    hit.t
                } else {
                    closest_so_far
                };
            }
        }
        maybe_hit
    }
}