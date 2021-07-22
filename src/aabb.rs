use crate::maths::Ray;
use crate::maths::Vec3;

#[derive(Copy, Clone)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    pub fn null_aabb() -> Aabb {
        Aabb::new(Vec3::zero(), Vec3::zero())
    }

    pub fn combine(self, other: Aabb) -> Aabb {
        let min = Vec3::new(
            f32::min(self.min.x, other.min.x),
            f32::min(self.min.y, other.min.y),
            f32::min(self.min.z, other.min.z),
        );

        let max = Vec3::new(
            f32::max(self.max.x, other.max.x),
            f32::max(self.max.y, other.max.y),
            f32::max(self.max.z, other.max.z),
        );

        Aabb::new(min, max)
    }

    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.dir[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let tmin = if t0 > tmin { t0 } else { tmin };

            let tmax = if t1 < tmax { t1 } else { tmax };

            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}
