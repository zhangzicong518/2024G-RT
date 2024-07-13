use crate::utils::*;

pub struct Interval {
    pub tmin: f64,
    pub tmax: f64,
}

impl Interval {
    pub fn new(tmin: f64, tmax: f64) -> Self {
        Self {
            tmin,
            tmax,
        }
    }

    pub fn new_union(a: Interval, b: Interval) -> Interval {
        Interval {
            tmin: fmin(a.tmin, b.tmin),
            tmax: fmax(a.tmax, b.tmax),
        }
    }

    pub fn new_overlap(a: Interval, b: Interval) -> Interval {
        Interval {
            tmin: fmax(a.tmin, b.tmin),
            tmax: fmin(a.tmax, b.tmax),
        }
    }

    pub fn default() -> Interval {
        Interval {
            tmin: f64::INFINITY,
            tmax: f64::NEG_INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.tmax - self.tmin
    }

    pub fn contains(&self, t: f64) -> bool {
        self.tmin <= t && t <= self.tmax
    }

    pub fn surrounds(&self, t: f64) -> bool {
        self.tmin < t  && t < self.tmax
    }

    pub fn clamp(&self, t: f64) -> f64 {
        if t > self.tmax {
            self.tmax
        } else if  t < self.tmin {
            self.tmin
        } else {
            t
        }
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval {
            tmin: self.tmin - padding,
            tmax: self.tmax + padding,
        }
    }
}

impl Clone for Interval {
    fn clone(&self) -> Self {
      Interval {
        tmin: self.tmin,
        tmax: self.tmax,
      }
    }
  }
  
  impl Copy for Interval {
  }


pub const empty_interval: Interval = Interval { 
    tmin: core::f64::INFINITY, 
    tmax: core::f64::NEG_INFINITY 
};

pub const universe_interval: Interval = Interval { 
    tmin: core::f64::NEG_INFINITY, 
    tmax: core::f64::INFINITY 
};