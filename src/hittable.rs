use crate::maths::Vec3;
use crate::maths::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    fn create(ray: &Ray, t: f32, outward_normal: Vec3) -> HitRecord {
        let point = ray.at(t);
        let front_face = Vec3::dot(ray.dir, outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => outward_normal * -1.0,
        };
        HitRecord {
            point,
            normal,
            t,
            front_face,
        }
    }
}
pub trait Hittable {
    fn hit(&self, tmin: f32, tmax: f32, ray: &Ray) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, tmin: f32, tmax: f32, ray: &Ray) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length2();
        let half_b = oc.dot(ray.dir);
        let c = oc.length2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = f32::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if root < tmin || tmax < root {
            root = (-half_b + sqrtd) / a;
            if root < tmin || tmax < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = point - self.center / self.radius;

        Some(HitRecord::create(ray, root, outward_normal))
    }
}
