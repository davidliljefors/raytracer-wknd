use std::cmp::Ordering;
use std::usize;
use rand::Rng;

use crate::aabb::Aabb;
use crate::hittable::*;

pub struct BvhNode {
    left: HittablePtr,
    right: HittablePtr,
    bounding_box: Aabb,
}

fn compare_x(a: &HittablePtr, b: &HittablePtr,) -> Ordering {
    Ordering::Greater
}
fn compare_y(a: &HittablePtr, b: &HittablePtr,) -> Ordering {
    Ordering::Greater
}
fn compare_z(a: &HittablePtr, b: &HittablePtr,) -> Ordering {
    Ordering::Greater
}

const NUMBERS: [fn(&HittablePtr, &HittablePtr)->Ordering;3] = [compare_x, compare_y, compare_z];

impl BvhNode {
    pub fn new(list: HittableList) -> BvhNode {
        BvhNode::create_next(list.objects(), 0, list.objects().len())
    }

    fn create_next(src_objects: &[HittablePtr], start: usize, end: usize) -> BvhNode {
        let mut rng = rand::thread_rng();
        let mut objects = src_objects.to_vec();
        let comparator = NUMBERS[rng.gen_range(0..=2)];
        let span = end - start;
        
        let left;
        let right;

        if span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        }
        else if span == 2 {
            if comparator(&objects[start], &objects[start+1]) == Ordering::Less {
                left = objects[start].clone();
                right = objects[start+1].clone();
            } else {
                left = objects[start+1].clone();
                right = objects[start].clone();
            }
        } else {
            let range = &mut objects[start..end];
            range.sort_by(comparator);
            let mid = start + span / 2;
            left = std::sync::Arc::new(BvhNode::create_next(src_objects, start, mid));
            right = std::sync::Arc::new(BvhNode::create_next(src_objects, mid, end));
        }

        BvhNode{left, right, bounding_box:Aabb::null_aabb() }
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
