use crate::utils::*;
use crate::vec3::*;
use crate::color::*;
use crate::interval::*;

use std::sync::Arc;
use std::path::Path;
use image::{DynamicImage, GenericImageView};

pub trait TextureTrait {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
    fn instancing(self) -> Arc<dyn TextureTrait + Send + Sync>;
}

pub struct SolidColor {
    pub albedo: Vec3,
}

impl SolidColor {
    pub fn new(albedo: Vec3) -> SolidColor {
        SolidColor {
            albedo,
        }
    }
}

impl TextureTrait for SolidColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.albedo
    }

    fn instancing(self) -> Arc<dyn TextureTrait + Send + Sync> {
        Arc::new(self)
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn TextureTrait + Send + Sync>,
    odd: Arc<dyn TextureTrait + Send + Sync>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn TextureTrait + Send + Sync>, odd: Arc<dyn TextureTrait + Send + Sync>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_from_color(scale: f64, even: Vec3, odd: Vec3) -> Self {
        Self{
            inv_scale: 1.0 / scale,
            even: SolidColor::new(even).instancing(),
            odd: SolidColor::new(odd).instancing(),
        }
    }
}

impl TextureTrait for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let xInteger = (self.inv_scale * p.x).floor() as i32;
        let yInteger = (self.inv_scale * p.y).floor() as i32;
        let zInteger = (self.inv_scale * p.z).floor() as i32;
        let is_even = (xInteger + yInteger + zInteger) % 2 == 0;
        if is_even { self.even.value(u, v, p) } else {self.odd.value(u, v, p) }
    }

    fn instancing(self) -> Arc<dyn TextureTrait + Send + Sync> {
        Arc::new(self)
    }
}

pub struct ImageTexture {
    img: DynamicImage,
}

impl ImageTexture {
    pub fn new(path: &Path) -> Self {
        let img = image::open(path).expect("File not found");
        ImageTexture { img }
    }
}

impl TextureTrait for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let (width, height) = (self.img.width(), self.img.height());
        if height == 0 || width == 0 {
            return Vec3::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0,1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0,1.0).clamp(v);

        let i = {
            let i = (u * width as f64) as u32;
            if i >= width {
                width - 1
            } else {
                i
            }
        };

        let j = {
            let j = (v * height as f64) as u32;
            if j >= height {
                height - 1
            } else {
                j
            }
        };

        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i, j);

        Vec3::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }

    fn instancing(self) -> Arc<dyn TextureTrait + Send + Sync> {
        Arc::new(self)
    }
}

