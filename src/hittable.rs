use crate::material::MaterialPtr;
use crate::maths::Ray;
use crate::maths::Vec3;

pub type HittablePtr = std::sync::Arc<dyn Hittable>;
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: MaterialPtr,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    fn create(ray: &Ray, t: f32, material: MaterialPtr, outward_normal: Vec3) -> HitRecord {
        let point = ray.at(t);
        let front_face = Vec3::dot(ray.dir, outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => outward_normal * -1.0,
        };
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}
pub trait Hittable: Send + Sync {
    fn hit(&self, tmin: f32, tmax: f32, ray: &Ray) -> Option<HitRecord>;
}   
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: MaterialPtr,
}

impl Sphere {
    pub fn create(x: f32, y: f32, z: f32, radius: f32, material: MaterialPtr) -> HittablePtr {
        std::sync::Arc::new(Sphere {
            center: Vec3 { x, y, z },
            radius,
            material,
        })
    }
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
        let outward_normal = (point - self.center) / self.radius;

        Some(HitRecord::create(
            ray,
            root,
            self.material.clone(),
            outward_normal,
        ))
    }
}

pub struct HittableList {
    objects: Vec<HittablePtr>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::<HittablePtr>::new(),
        }
    }

    pub fn add(&mut self, hittable: HittablePtr) {
        self.objects.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, tmin: f32, tmax: f32, ray: &Ray) -> Option<HitRecord> {
        let mut closest_distance = tmax;
        let mut temphit = None;
        for hittable in self.objects.as_slice() {
            if let Some(hit) = hittable.hit(tmin, closest_distance, ray) {
                closest_distance = hit.t;
                temphit = Some(hit);
            }
        }
        temphit
    }
}
