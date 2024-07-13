use std::sync::Arc;

use crate::util::*;
use crate::vec3::*;
use crate::hitable::*;

pub struct BVHnode {
    pub left: Arc<dyn Hittable + Send + Sync>,
    pub right: Arc<dyn Hittable + Send + Sync>,
    pub bbox: Aabb,
}

impl BVHnode {
    pub fn new(list &mut Hittable_list) -> Self {
        new_from_list(&mut list.objects, 0, list.objects.len());
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
                left = Bvhnode::new_from_list(objects, start, mid).instancing();
                right = Bvhnode::new_from_list(objects, mid, end).instancing();
            }
        }
        Self {
            left,
            right,
            bbox,
        }
    }

    fn bbox_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
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
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, if hit_left {rec.t} else {ray_t.max}), rec);
        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn instancing(self) -> Arc<dyn Hittable + Send + Sync> {
        Arc::new(self)
    }
}