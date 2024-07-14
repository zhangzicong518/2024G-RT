use crate::utils::*;
use crate::vec3::*;
use crate::material::*;
use crate::aabb::*;
use crate::hitable::*;
use crate::interval::*;
use crate::ray::*;

use std::sync::Arc;

pub struct Quad {
    Q: Vec3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn MaterialTrait + Send + Sync>,
    bbox: Aabb,
    normal: Vec3,
    D: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(Q: Vec3, u: Vec3, v: Vec3, material: Arc<dyn MaterialTrait + Send + Sync>) -> Self {
        let bbox_diagonal1 = Aabb::new_from_point(Q, Q + u + v);
        let bbox_diagonal2 = Aabb::new_from_point(Q + u, Q + v);
        let bbox = Aabb::new_from_bbox(bbox_diagonal1, bbox_diagonal2);
        let n = u.cross(v);
        let normal = unit_vec(n);
        let D = normal * Q;
        let w = n / (n * n);
        Self {
            Q,
            u,
            v,
            material,
            bbox,
            normal,
            D,
            w,
        }
    }

    pub fn is_interier(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
    
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
          false
        } 
        else {
          rec.u = a;
          rec.v = b;
          true
        }
    }
}

impl Clone for Quad {
    fn clone(&self) -> Self {
      Self {
        material: self.material.clone(),
        ..*self
      }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
       let denom = self.normal * ray.direction();
       if fabs(denom) < 1e-8 {
        return false;
       }
       let t = (self.D - self.normal * ray.origin()) / denom;
       if !ray_t.contains(t) {
        return false;
       }
       let intersection = ray.at(t);
       let planar_hitpt_vector = intersection - self.Q;
       let alpha = self.w  * planar_hitpt_vector.cross(self.v);
       let beta = self.w * self.u.cross(planar_hitpt_vector);

        
       if !self.is_interier(alpha, beta, rec) {
            return false;
        }

        *rec = HitRecord::new(
            t,
            intersection,
            self.normal,
            self.material.clone(),
            true,
            rec.u, 
            rec.v
        );
        rec.set_face_normal(*ray, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn instancing(self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self)
    }
}

pub fn create_box(a: Vec3, b: Vec3, material: Arc<dyn MaterialTrait + Send + Sync>) -> Hittable_list {
    let mut sides = Hittable_list::default();
    let min = Vec3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Vec3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
  
    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);
  
    sides.add(Quad::new(
            Vec3::new(min.x, min.y, max.z),
            dx, 
            dy, 
            material.clone()
        ).instancing()
    );

    sides.add(Quad::new(
            Vec3::new(max.x, min.y, max.z),
            dz * -1.0, 
            dy, 
            material.clone()
        ).instancing()
    );
  
    sides.add(Quad::new(
            Vec3::new(max.x, min.y, min.z),
            dx * -1.0, 
            dy, 
            material.clone()
        ).instancing()
    ); 
  
    sides.add(Quad::new(
            Vec3::new(min.x, min.y, min.z),
            dz, 
            dy, 
            material.clone()
        ).instancing()
    ); 
  
    sides.add(Quad::new(
            Vec3::new(min.x, max.y, max.z),
            dx, 
            dz * -1.0, 
            material.clone()
        ).instancing()
    );

    sides.add(Quad::new(
            Vec3::new(min.x, min.y, min.z),
            dx, 
            dz, 
            material.clone()
        ).instancing()
    ); 

    sides
  }