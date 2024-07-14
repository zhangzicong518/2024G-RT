use crate::interval::{Interval, empty_interval, universe_interval};
use crate::vec3::*;
use crate::utils::*;

use std::ops::{Add};

pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {

    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb {
            x,
            y,
            z,
        }.pad_to_minimums()
    }

    pub fn default() -> Aabb{
        Aabb {
          x: Interval::default(),
          y: Interval::default(),
          z: Interval::default(),
        }
    }

    pub fn new_from_point(a: Vec3, b: Vec3) -> Aabb {
        let x = Interval::new(fmin(a.x, b.x), fmax(a.x, b.x));
        let y = Interval::new(fmin(a.y, b.y), fmax(a.y, b.y));
        let z = Interval::new(fmin(a.z, b.z), fmax(a.z, b.z));
        Aabb {
            x,
            y,
            z,
        }.pad_to_minimums()
    }

    pub fn new_from_bbox(a: Aabb, b: Aabb) -> Aabb {
        Aabb {
            x: Interval::new_union(a.x, b.x),
            y: Interval::new_union(a.y, b.y),
            z: Interval::new_union(a.z, b.z),
        }.pad_to_minimums()
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("invalid indexing"),
        }
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return 0;
            }
            else {
                return 2;
            }
        }
        else {
            if self.y.size() > self.z.size() {
                return 1;
            }
            else {
                return 2;
            }
        };
        0
    }

    pub fn hit(&self, r: &Ray, ray_t: Interval) -> bool {
        let ray_origin = r.origin();
        let ray_direction = r.direction();
        
        for i in 0..3 {
            let ax = self.axis_interval(i);
            let adinv = 1.0 / ray_direction.lp(i as u8);

            let t0 = (ax.tmin - ray_origin.lp(i as u8)) * adinv;
            let t1 = (ax.tmax - ray_origin.lp(i as u8)) * adinv;

            let lef = fmax(ray_t.tmin, fmin(t0, t1));
            let rig = fmin(ray_t.tmax, fmax(t0, t1));

            if rig <= lef {
                return false
            }
        }
        true 
    }

    pub fn pad_to_minimums(&mut self) -> Self {
        let delta = 0.0001;
        if self.x.size() < delta { 
            self.x = self.x.expand(delta); 
        }
        if self.y.size() < delta { 
            self.y = self.y.expand(delta); 
        }
        if self.z.size() < delta { 
            self.z = self.z.expand(delta); 
        }
        *self
    }
}

impl Clone for Aabb {
    fn clone(&self) -> Self {
      Self {
        ..*self
      }
    }
  }
  
  impl Copy for Aabb {
  
  }

pub const empty: Aabb = Aabb { 
    x: empty_interval,
    y: empty_interval,
    z: empty_interval,
};

pub const universe: Aabb = Aabb { 
    x: universe_interval,
    y: universe_interval,
    z: universe_interval,
};

impl Add<Vec3> for Aabb {
    type Output = Aabb;
    fn add(self, offset: Vec3) -> Aabb {
      Aabb::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}
  
  
impl Add<Aabb> for Vec3 {
    type Output = Aabb;
    fn add(self, bbox: Aabb) -> Aabb {
      bbox + self
    }
}