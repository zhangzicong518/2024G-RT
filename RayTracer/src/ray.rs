pub use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub a_origin: Vec3,
    pub b_direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(a_origin: Vec3, b_direction: Vec3, time: f64) -> Self {
        Self {
            a_origin,
            b_direction,
            time,
        }
    }

    pub fn default() -> Self {
        Self {
            a_origin: Vec3::zero(),
            b_direction: Vec3::zero(),
            time: 0.0,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.a_origin
    }

    pub fn direction(&self) -> Vec3 {
        self.b_direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.a_origin + self.b_direction * t
    }
    pub fn info(&self){
        println!("ori");
        self.a_origin.info();
        println!("dir");
        self.b_direction.info();
    }
}
