use rand::Rng;
use std::cmp::Ordering;
use std::usize;

use crate::aabb::Aabb;
use crate::hittable::*;

pub struct BvhNode {
    left: HittablePtr,
    right: HittablePtr,
    bounding_box: Aabb,
}

fn compare_by_axis(a: &HittablePtr, b: &HittablePtr, axis: usize) -> Ordering {
    let left = a.bounding_box().expect("Bbox null in ctor");
    let right = b.bounding_box().expect("Bbox null in ctor");
    left.min[axis]
        .partial_cmp(&right.min[axis])
        .expect("Bbox error")
}

impl BvhNode {
    pub fn new(list: HittableList) -> BvhNode {
        BvhNode::create_next(list.objects(), 0, list.objects().len())
    }

    fn create_next(src_objects: &[HittablePtr], start: usize, end: usize) -> BvhNode {
        
        let mut objects = src_objects.to_vec();
        let axis = rand::thread_rng().gen_range(0..=2);
        let comparator = move |a:&HittablePtr, b:&HittablePtr| compare_by_axis(a, b, axis);
        let span = end - start;

        let left;
        let right;

        if span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            let range = &mut objects[start..end];
            range.sort_by(comparator);
            let mid = start + span / 2;
            left = std::sync::Arc::new(BvhNode::create_next(src_objects, start, mid));
            right = std::sync::Arc::new(BvhNode::create_next(src_objects, mid, end));
        }

        if let (Some(left_box), Some(right_box)) = (left.bounding_box(), right.bounding_box()) {
            let bounding_box = Aabb::combine(left_box, right_box);
            return BvhNode {
                left,
                right,
                bounding_box,
            };
        }

        panic!("No bounding box in bvh ctor");
    }
}

impl Hittable for BvhNode {
    fn hit(&self, tmin: f32, tmax: f32, ray: &crate::maths::Ray) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, tmin, tmax) {
            return None;
        }

        let hit_left = self.left.hit(tmin, tmax, ray);
        let hit_right = self.right.hit(tmin, tmax, ray);

        match (&hit_left, &hit_right) {
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    hit_left
                } else {
                    hit_right
                }
            }
            (Some(_), None) => hit_left,
            (None, Some(_)) => hit_right,
            (None, None) => None,
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bounding_box)
    }
}
