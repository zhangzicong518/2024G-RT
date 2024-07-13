use crate::hitable::*;
use crate::vec3::*;
use crate::material::*;
use crate::aabb::*;
use crate::utils::*;
use crate::interval::*;
use crate::{ray::Ray, vec3::Vec3};

use std::sync::Arc;
use std::f64::consts::PI;


pub struct Sphere {
    pub center1: Vec3,
    pub radius: f64,
    pub material: Arc<dyn MaterialTrait + Send + Sync>,
    pub is_moving: bool,
    pub center_vec: Vec3,
    pub bbox: Aabb,
}

impl Sphere {
    pub fn new(center1: Vec3, radius: f64, material: Arc<dyn MaterialTrait + Send + Sync>) -> Sphere {
        let rvec = Vec3::new(fmax(radius, 0.0), fmax(radius, 0.0), fmax(radius, 0.0));
        Sphere {
            center1,
            radius,
            material,
            is_moving: false,
            center_vec: Vec3::zero(),
            bbox: Aabb::new_from_point(center1 - rvec, center1 + rvec),
        }
    }

    pub fn new_moving(center1: Vec3, center2: Vec3, radius: f64, material: Arc<dyn MaterialTrait + Send + Sync>) -> Sphere {
        let rvec = Vec3::new(fmax(radius, 0.0), fmax(radius, 0.0), fmax(radius, 0.0));
        let bbox1 = Aabb::new_from_point(center1 - rvec, center1 + rvec);
        let bbox2 = Aabb::new_from_point(center2 - rvec, center2 + rvec);
        Sphere {
            center1,
            radius,
            material,
            is_moving: true,
            center_vec: center2-center1,
            bbox: Aabb::new_from_bbox(bbox1, bbox2),
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    pub fn sphere_center(&self, time: f64) -> Vec3 {
        self.center1 + self.center_vec * time
    }

    pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }

}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord, ) -> bool {
        let center = if self.is_moving {
            self.sphere_center(r.time())
        }
        else {
            self.center1
        };
        let vec = center - r.origin();
        let a = r.direction().squared_length();
        let h = (r.direction() * vec);
        let c = vec.squared_length() - self.radius *self.radius;
        let delta = h * h - a * c;

        if delta < 0.0 {
            return false;
        }

        let mut temp = (h-delta.sqrt()) / a;
        if (!ray_t.surrounds(temp)) {
            temp = (h + delta.sqrt()) / a;
            if (!ray_t.surrounds(temp)) {
                return false;
            }
        }

        rec.t = temp;
        rec.point = r.at(temp);
        rec.front_face = (rec.point - center) * r.direction() < 0.0;
        rec.normal = if rec.front_face {
                        (rec.point - center) * (1.0 / self.radius)
                    } 
                    else {
                        (center - rec.point) * (1.0 / self.radius)
                    };
        rec.material = Arc::clone(&self.material);
        Self::get_sphere_uv(rec.normal, &mut rec.u, &mut rec.v);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn instancing(self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self)
    }
}
