use std::sync::Arc;
use std::cmp::Ordering;

use crate::utils::*;
use crate::vec3::*;
use crate::hitable::*;
use crate::interval::*;
use crate::aabb::*;

pub struct BVHnode {
    pub left: Arc<dyn Hittable + Send + Sync>,
    pub right: Arc<dyn Hittable + Send + Sync>,
    pub bbox: Aabb,
}

impl BVHnode {
    pub fn new(list :&mut Hittable_list) -> Self {
        let end = list.objects.len();
        Self::new_from_list(&mut list.objects, 0, end)
    }

    pub fn new_from_list(objects : &mut Vec<Arc<dyn Hittable + Send + Sync>>, start: usize, end: usize) -> Self {
        let mut bbox = Aabb::default();
        for i in start..end {
            bbox = Aabb::new_from_bbox(bbox, objects[i].bounding_box());
        }

        let axis = bbox.longest_axis();

        let object_span = end - start;
        let left: Arc<dyn Hittable + Send + Sync>;
        let right: Arc<dyn Hittable + Send + Sync>;

        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            },
            2 => {
                left = objects[start].clone();
                right = objects[start+1].clone();
            },
            _ => {
                objects[start..end].sort_by(|a, b| Self::bbox_compare(a, b, axis));
                let mid = (start + end) / 2;
                left = BVHnode::new_from_list(objects, start, mid).instancing();
                right = BVHnode::new_from_list(objects, mid, end).instancing();
            }
        }
        Self {
            left,
            right,
            bbox,
        }
    }

    fn bbox_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>, axis_index: usize) -> Ordering {
        let a_binding = a.bounding_box();
        let a_axis_interval = a_binding.axis_interval(axis_index);
        let b_binding = b.bounding_box();
        let b_axis_interval = b_binding.axis_interval(axis_index);
        if a_axis_interval.tmin < b_axis_interval.tmin {
            Ordering::Less
        }
        else if a_axis_interval.tmin == b_axis_interval.tmin {
            Ordering::Equal
        }
        else {
            Ordering::Greater
        }
    }
}

impl Hittable for BVHnode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(r, Interval::new(ray_t.tmin, if hit_left {rec.t} else {ray_t.tmax}), rec);
        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn instancing(self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self)
    }
}