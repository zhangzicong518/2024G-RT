use crate::{ray::Ray, vec3::Vec3};
use crate::interval::*;
use crate::material::*;
use crate::aabb::*;

use std::sync::Arc;

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn MaterialTrait + Send + Sync>,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval, res: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
    fn instancing(self) -> Arc<dyn Hittable + Send + Sync>; 
}

impl HitRecord {
    pub fn new(t: f64, point: Vec3, normal: Vec3, material: Arc<dyn MaterialTrait + Send + Sync>, front_face: bool, u: f64, v: f64) -> HitRecord {
        HitRecord {
            t,
            point,
            normal,
            material,
            front_face,
            u,
            v,
        }
    }

    pub fn default() -> Self {
        HitRecord {
            t: 0.0,
            point: Vec3::zero(),
            normal: Vec3::zero(),
            material: Lambertian::new_from_color(Vec3::zero()).instancing(),
            front_face: false,
            u: 0.0,
            v: 0.0,
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

impl Clone for HitRecord {
    fn clone(&self) -> Self {
      HitRecord {
        material: self.material.clone(),
        ..*self
      }
    }
}

pub struct Hittable_list {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub bbox: Aabb,
}

impl Hittable_list {
    pub fn new(objects: Vec<Arc<dyn Hittable + Send + Sync>>) -> Hittable_list {
        let mut bbox = Aabb::default();
        for iter in &objects {
            bbox = Aabb::new_from_bbox(bbox, iter.bounding_box());
        }
        Hittable_list {
            objects,
            bbox,
        }
    }

    pub fn default() -> Hittable_list {
        Hittable_list {
          objects: Vec::default(),
          bbox: Aabb::default(),
        }    
      }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.bbox = Aabb::new_from_bbox(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for Hittable_list {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut rec_tmp = HitRecord::default();
        let mut closest_so_far = ray_t.tmax;
        let mut hit_anything = false;

        for object in &self.objects {
            if object.hit(&ray, &Interval::new(ray_t.tmin, closest_so_far), &mut rec_tmp) {
                hit_anything =  true;
                closest_so_far = rec_tmp.t;
                *rec = rec_tmp.clone();
            }
        }
        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn instancing(self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self)
    }
}