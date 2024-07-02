use std::collections::HashMap;

use nalgebra::{Matrix4, Vector3, Vector4};
use crate::triangle::Triangle;
use std::time::Instant;

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default, Clone)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    /*  You may need to uncomment here to implement the MSAA method  */
    frame_sample: Vec<Vector3<f64>>,
    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

pub const antialiasing_method:&str = "FXAA";

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;

        match antialiasing_method {
            
            "MSAA" => {   
                r.frame_buf.resize((w * h) as usize, Vector3::zeros());
                r.depth_buf.resize((w * h * 4) as usize, f64::MAX);
                r.frame_sample.resize((w * h * 4) as usize, Vector3::zeros());
            },
        
            "FXAA" => {
                r.frame_buf.resize((w * h) as usize, Vector3::zeros());
                r.depth_buf.resize((w * h) as usize, f64::MAX);                
            },

            _ => {
                r.frame_buf.resize((w * h) as usize, Vector3::zeros());
                r.depth_buf.resize((w * h) as usize, f64::MAX);                
            }
        }

        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize
    }

    fn set_pixel(&mut self, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (self.height as f64 - 1.0 - point.y) * self.width as f64 + point.x;
        self.frame_buf[ind as usize] = *color;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color => {
                match antialiasing_method {
                    "MSAA" => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                        self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
                    },
                    "FXAA" => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                    },
                    _ => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                    },
                }
            }
            Buffer::Depth => {
                self.depth_buf.fill(f64::MAX);
            }
            Buffer::Both => {
                match antialiasing_method {
                    "MSAA" => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));                
                        self.frame_sample.fill(Vector3::new(0.0, 0.0, 0.0));
                        self.depth_buf.fill(f64::MAX);
                    },
                    "FXAA" => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                        self.depth_buf.fill(f64::MAX);
                    }
                    _ => {
                        self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                        self.depth_buf.fill(f64::MAX);
                    },
                }
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(&mut self, pos_buffer: PosBufId, ind_buffer: IndBufId, col_buffer: ColBufId, _typ: Primitive) {
        let buf = &self.clone().pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.clone().ind_buf[&ind_buffer.0];
        let col = &self.clone().col_buf[&col_buffer.0];

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;

        let mvp = self.projection * self.view * self.model;

        let now = Instant::now();

        for i in ind {
            let mut t = Triangle::new();
            let mut v =
                vec![mvp * to_vec4(buf[i[0]], Some(1.0)), // homogeneous coordinates
                     mvp * to_vec4(buf[i[1]], Some(1.0)), 
                     mvp * to_vec4(buf[i[2]], Some(1.0))];
    
            for vec in v.iter_mut() {
                *vec = *vec / vec.w;
            }
            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0);
                vert.z = vert.z * f1 + f2;
            }
            for j in 0..3 {
                // t.set_vertex(j, Vector3::new(v[j].x, v[j].y, v[j].z));
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
                t.set_vertex(j, v[j]);
            }
            let col_x = col[i[0]];
            let col_y = col[i[1]];
            let col_z = col[i[2]];
            t.set_color(0, col_x[0], col_x[1], col_x[2]);
            t.set_color(1, col_y[0], col_y[1], col_y[2]);
            t.set_color(2, col_z[0], col_z[1], col_z[2]);

            self.rasterize_triangle(&t);
        }

        let elapsed = now.elapsed(); 
        println!("Time taken to render: {:?}", elapsed);

    }

    pub fn Normal_sampling(&mut self,t: &Triangle) {
        let pos = &t.to_vector4();
        let lef_x = pos[0].x.min(pos[1].x).min(pos[2].x) as usize;
        let rig_x = pos[0].x.max(pos[1].x).max(pos[2].x) as usize;
        let low_y = pos[0].y.min(pos[1].y).min(pos[2].y) as usize;
        let hig_y = pos[0].y.max(pos[1].y).max(pos[2].y) as usize;
        let mut v: [Vector3<f64>; 3] = [
            Vector3::new(t.v[0].x, t.v[0].y, t.v[0].z),
            Vector3::new(t.v[1].x, t.v[1].y, t.v[1].z),
            Vector3::new(t.v[2].x, t.v[2].y, t.v[2].z),
        ];

        for x in lef_x..=rig_x {
            for y in low_y..=hig_y {
                let (fx,fy) = (x as f64, y as f64);
                if !inside_triangle(fx + 0.5, fy + 0.5, &v) {
                    continue;
                }
                let (a, b, c) = compute_barycentric2d(fx + 0.5, fy + 0.5, &v);
                let w_reciprocal = 1.0 / (a / pos[0].w + b / pos[1].w + c / pos[2].w);
                let mut z_interpolated = a * pos[0].z / pos[0].w + b * pos[1].z / pos[1].w + c * pos[2].z / pos[2].w;
                z_interpolated *= w_reciprocal;
                
                if z_interpolated < self.depth_buf[self.get_index(x, y )] {
                    let index = self.get_index(x, y);
                    self.depth_buf[index] = z_interpolated;
                    self.set_pixel(&Vector3::new(fx, fy, 1.0), &t.get_color());
                }
            }
        }
    }

    pub fn MSAA_sampling (&mut self,t: &Triangle) {
        // println!("MSAA working");
        let pos = &t.to_vector4();
        let lef_x = pos[0].x.min(pos[1].x).min(pos[2].x) as usize;
        let rig_x = pos[0].x.max(pos[1].x).max(pos[2].x) as usize;
        let low_y = pos[0].y.min(pos[1].y).min(pos[2].y) as usize;
        let hig_y = pos[0].y.max(pos[1].y).max(pos[2].y) as usize;
        let mut v: [Vector3<f64>; 3] = [
            Vector3::new(t.v[0].x, t.v[0].y, t.v[0].z),
            Vector3::new(t.v[1].x, t.v[1].y, t.v[1].z),
            Vector3::new(t.v[2].x, t.v[2].y, t.v[2].z),
        ];

        let posx = [0.25, 0.25, 0.75, 0.75];
        let posy = [0.25, 0.75, 0.25, 0.75];

        for x in lef_x..=rig_x {
            for y in low_y..=hig_y {
                let mut depth_count = 0;
                let index = self.get_index(x, y);
                for id in 0..4 {
                    let (fx,fy) = (x as f64 + posx[id], y as f64 + posy[id] );
                    if !inside_triangle(fx as f64, fy as f64, &v) {
                        continue;
                    }
                    let (a, b, c) = compute_barycentric2d(fx as f64, fy as f64, &v);
                    let w_reciprocal = 1.0 / (a / pos[0].w + b / pos[1].w + c / pos[2].w);
                    let mut z_interpolated = a * pos[0].z / pos[0].w + b * pos[1].z / pos[1].w + c * pos[2].z / pos[2].w;
                    z_interpolated *= w_reciprocal;
                    
                    if z_interpolated < self.depth_buf[index * 4 + id] {
                        let color = t.color[0] * 255.0;
                        self.frame_sample[index * 4 + id] = color;
                        self.depth_buf[index * 4 + id] = z_interpolated;
                        depth_count += 1;
                    }
                }

                if depth_count > 0 {
                    //println!("MSAA working");
                    self.frame_buf[index] = Vector3::zeros();
                    for id in 0..4 {
                        self.frame_buf[index] += self.frame_sample[index * 4 + id] / 4.0;
                    }
                }
            }
        }
    }

    pub fn FXAA_sampling (&mut self,t: &Triangle) {
        self.Normal_sampling(t);
        let pos = &t.to_vector4();
        let lef_x = pos[0].x.min(pos[1].x).min(pos[2].x) as usize;
        let rig_x = pos[0].x.max(pos[1].x).max(pos[2].x) as usize;
        let low_y = pos[0].y.min(pos[1].y).min(pos[2].y) as usize;
        let hig_y = pos[0].y.max(pos[1].y).max(pos[2].y) as usize;

        for x in lef_x..=rig_x {
            for y in low_y..=hig_y {
                if self.is_edge(x, y, &t) {
                    self.smooth_pixel(x, y);
                }
            }
        }
    }

    pub fn is_edge(&self, x: usize, y: usize, t: &Triangle) -> bool {
        let edge_threshold = 0.2; 
        let center_depth = self.depth_buf[self.get_index(x, y)];
    
        let mut edge = false;
        for i in 0..3 {
            let neighbor_depth = self.depth_buf[self.get_index(t.v[i].x as usize, t.v[i].y as usize)];
            let depth_diff = (neighbor_depth - center_depth).abs();
            if depth_diff > edge_threshold {
                edge = true;
                break;
            }
        }
    
        edge
    }
    
    pub fn smooth_pixel(&mut self, x: usize, y: usize) {
        let mut total_color = Vector3::zeros();
        let mut count = 0;
    
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if nx < self.width as usize && ny < self.height as usize {
                    total_color += self.frame_buf[self.get_index(nx, ny)];
                    count += 1;
                }
            }
        }
    
        if count > 0 {
            let index = self.get_index(x, y);
            self.frame_buf[index] = total_color / (count as f64);
        }
    }

    pub fn rasterize_triangle(&mut self, t: &Triangle) {
        match antialiasing_method {
            "MSAA" => self.MSAA_sampling(t),
            "FXAA" => self.FXAA_sampling(t),
            _ => self.Normal_sampling(t),
        }
    }

    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
}

fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> bool {
    let p = Vector3::new(x, y, 0.0);
    let ap = p - v[0];
    let bp = p - v[1];
    let cp = p - v[2];
    let ab = v[1] - v[0];
    let bc = v[2] - v[1];
    let ca = v[0] - v[2];
    let c1 = ab.cross(&ap);
    let c2 = bc.cross(&bp);
    let c3 = ca.cross(&cp);
    (c1.z > 0.0 && c2.z > 0.0 && c3.z > 0.0) || (c1.z < 0.0 && c2.z < 0.0 && c3.z < 0.0)
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y - v[1].x * v[0].y);
    (c1, c2, c3)
}