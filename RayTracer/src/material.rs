use crate::vec3::*;
use crate::utils::*;
use crate::hitable::*;
use crate::ray::*;
use crate::interval::*;
use crate::texture::*;

use std::rc::Rc;
use std::sync::Arc;

pub trait MaterialTrait {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool{
        false
    }
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn instancing(self) -> Arc<dyn MaterialTrait + Send + Sync>;
}

pub struct Lambertian {
    pub tex: Arc<dyn TextureTrait + Send + Sync>,
}

impl Lambertian {
    pub fn new(tex: Arc<dyn TextureTrait + Send + Sync>) -> Self {
        Self{
            tex,
        }
    }

    pub fn new_from_color(albedo: Vec3) -> Lambertian {
        Lambertian{
            tex: SolidColor::new(albedo).instancing(),
        }
    }
}

impl MaterialTrait for Lambertian {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + random_in_unit_shpere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.point, scatter_direction, r.time);
        *attenuation = self.tex.value(hit_record.u, hit_record.v, hit_record.point);
        true
    }

    fn instancing(self) -> Arc<dyn MaterialTrait + Send + Sync> {
        Arc::new(self)
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

impl MaterialTrait for Metal {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut reflected = reflect(r.direction(), hit_record.normal);
        reflected = unit_vec(reflected) + unit_vec(random_in_unit_shpere()) * self.fuzz;
        *scattered = Ray::new(hit_record.point, reflected, r.time());
        *attenuation = self.albedo;
        reflected * hit_record.normal > 0.0
    }

    fn instancing(self) -> Arc<dyn MaterialTrait + Send + Sync> {
        Arc::new(self)
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

impl MaterialTrait for Dielectric {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ratio =  if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let refracted = refract(unit_vec(r.direction()), hit_record.normal, ratio);
        *scattered = Ray::new(hit_record.point, refracted, r.time());
        true
    }

    fn instancing(self) -> Arc<dyn MaterialTrait + Send + Sync> {
        Arc::new(self)
    }
}

pub struct Diffuselight {
    pub tex: Arc<dyn TextureTrait + Send + Sync>,
}

impl Diffuselight {
    pub fn new(tex: Arc<dyn TextureTrait + Send + Sync>) -> Self {
        Self{
            tex,
        }
    }

    pub fn new_from_color(emit: Vec3) -> Self {
        Self{
            tex: SolidColor::new(emit).instancing(),
        }
    }
}

impl MaterialTrait for Diffuselight {
    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.tex.value(u, v, p)
    }

    fn instancing(self) -> Arc<dyn MaterialTrait + Send + Sync> {
        Arc::new(self)
    }
}

pub struct Isotropic {
    pub tex: Arc<dyn TextureTrait + Send + Sync>,
}

impl Isotropic {
    pub fn new(tex: Arc<dyn TextureTrait + Send + Sync>) -> Self {
        Self{
            tex,
        }
    }

    pub fn new_from_color(albedo: Vec3) -> Self {
        Self{
            tex: SolidColor::new(albedo).instancing(),
        }
    }
}

impl MaterialTrait for Isotropic {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *scattered = Ray::new(hit_record.point, unit_vec(random_in_unit_shpere()), r.time());
        *attenuation = self.tex.value(hit_record.u, hit_record.v, hit_record.point);
        true
    }

    fn instancing(self) -> Arc<dyn MaterialTrait + Send + Sync> {
        Arc::new(self)
    }
}
