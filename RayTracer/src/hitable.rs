use crate::{ray::Ray, vec3::Vec3};
use crate::interval::*;
use crate::material::*;
use crate::aabb::*;
use crate::bvh::*;
use crate::utils::*;
use crate::texture::*;

use std::sync::Arc;
use std::f64::consts::{PI, E};

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
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
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

    pub fn to_bvh(&mut self) -> Arc<dyn Hittable + Send + Sync> {
        (BVHnode::new(self)).instancing()
    }
}

impl Hittable for Hittable_list {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec_tmp = HitRecord::default();
        let mut closest_so_far = ray_t.tmax;
        let mut hit_anything = false;

        for object in &self.objects {
            if object.hit(&ray, Interval::new(ray_t.tmin, closest_so_far), &mut rec_tmp) {
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

pub struct Translate {
    pub objects: Arc<dyn Hittable + Send + Sync>,
    pub offset: Vec3,
    pub bbox: Aabb,
}

impl Translate {
    pub fn new(objects: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Translate {
        let mut bbox = Aabb::default();
        bbox = objects.bounding_box() + offset;
        Translate {
            objects,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
       let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
       if !self.objects.hit(&offset_ray, ray_t, rec) {
            return false;
        }
        rec.point += self.offset;
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn instancing(self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self)
    }
}

pub struct RotateY {
    pub objects: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f64,
    cos_theta: f64,
    pub bbox: Aabb,
}

impl RotateY {
    pub fn new(objects: Arc<dyn Hittable + Send + Sync>, angle: f64) -> RotateY {
        let radians = angle /180.0 * PI;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = objects.bounding_box();

        let mut min = Vec3::new(core::f64::INFINITY , core::f64::INFINITY , core::f64::INFINITY );
        let mut max = Vec3::new(core::f64::NEG_INFINITY, core::f64::NEG_INFINITY, core::f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = if i == 1 { bbox.x.tmax } else { bbox.x.tmin};
                    let y = if j == 1 { bbox.y.tmax } else { bbox.y.tmin};
                    let z = if k == 1 { bbox.z.tmax } else { bbox.z.tmin};
                    
                    let tester = Vec3::new(
                        cos_theta * x + sin_theta * z,
                        y,
                        -sin_theta * x + cos_theta * z,
                    );
                    min.x = fmin(min.x,tester.x);
                    max.x = fmax(max.x,tester.x);
                    min.y = fmin(min.y,tester.y);
                    max.y = fmax(max.y,tester.y);
                    min.z = fmin(min.z,tester.z);
                    max.z = fmax(max.z,tester.z);
                    
                }   
            }
        }
        let bbox = Aabb::new_from_point(min, max);
        Self {
            objects,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut origin = ray.origin();
        origin = Vec3::new(
            self.cos_theta * origin.x - self.sin_theta * origin.z,
            origin.y,
            self.sin_theta * origin.x + self.cos_theta * origin.z,
        );

        let mut direction = ray.direction();
        direction = Vec3::new(
            self.cos_theta * direction.x - self.sin_theta * direction.z,
            direction.y,
            self.sin_theta * direction.x + self.cos_theta * direction.z,
        );
    
        let rotated_ray = Ray::new(origin, direction, ray.time());
    
        if !self.objects.hit(&rotated_ray, ray_t, rec) {
          return false;
        }
        
        let mut point = rec.point;
        point = Vec3::new(
            self.cos_theta * point.x + self.sin_theta * point.z,
            point.y,
            -self.sin_theta * point.x + self.cos_theta * point.z,
        );

        let mut normal = rec.normal;
        normal = Vec3::new(
            self.cos_theta * normal.x + self.sin_theta * normal.z,
            normal.y,
            -self.sin_theta * normal.x + self.cos_theta * normal.z,
        );

        rec.point = point;
        rec.normal = normal;
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn instancing(self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self)
    }
}

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable + Send + Sync>,
    pub neg_inv_density: f64,
    pub phase_function: Arc<dyn MaterialTrait + Send + Sync>
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable + Send + Sync>, density: f64, tex: Arc<dyn TextureTrait + Send + Sync>) -> ConstantMedium {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new(tex).instancing(),
        }
    }

    pub fn new_from_color(boundary: Arc<dyn Hittable + Send + Sync>, density: f64, albedo: Vec3) -> ConstantMedium {
        Self::new(boundary, density, SolidColor::new(albedo).instancing())
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();


        if !self.boundary.hit(ray, Interval::new(core::f64::NEG_INFINITY, core::f64::INFINITY), &mut rec1) {
            return false;
        }

        if !self.boundary.hit(ray, Interval::new(rec1.t + 0.0001, core::f64::INFINITY), &mut rec2) {
            return false;
        }

        rec1.t = ray_t.clamp(rec1.t);
        rec2.t = ray_t.clamp(rec2.t);

        if rec1.t >= rec2.t {
            return false;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = ray.direction().length();
        let dis_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f64_range(0.0001, 1.0 - 0.0001).log(E);

        if hit_distance > dis_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.point = ray.at(rec.t);
        rec.material = self.phase_function.clone();
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }

    fn instancing(self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self)
    }
}

