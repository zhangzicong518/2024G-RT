use crate::vec3::*;
use crate::utils::*;
use crate::hitable::*;
use crate::ray::*;
use crate::interval::*;

use std::rc::Rc;

pub trait Material {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian{
            albedo: albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_direction = hit_record.normal + random_in_unit_shpere();
        if scatter_direction.near_zero() {
            Some((
                self.albedo, 
                Ray::new(hit_record.point, hit_record.normal, 0.0)
            ))
        }
        else {
            Some((
                self.albedo, 
                Ray::new(hit_record.point, scatter_direction, 0.0)
            ))
        }
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut reflected = reflect(r.direction(), hit_record.normal);
        reflected = unit_vec(reflected) + unit_vec(random_in_unit_shpere()) * self.fuzz;
        Some((self.albedo, Ray::new(hit_record.point, reflected, 0.0))) 
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric {
            refraction_index: refraction_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ratio =  if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let refracted = refract(unit_vec(r.direction()), hit_record.normal, ratio);
        Some((attenuation, Ray::new(hit_record.point, refracted, 0.0)))
    }
}
