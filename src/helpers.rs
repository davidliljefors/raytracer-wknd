use crate::color;
use crate::maths::*;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
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
trait Hittable {
    fn hit(&self, tmin: f32, tmax: f32, ray: &Ray) -> Option<HitRecord>;
}

struct Sphere {
    center: Vec3,
    radius: f32,
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

impl Vec3 {
    pub fn as_color(self: Vec3) -> color::Color {
        let r = (self.x * 255.999) as u8;
        let g = (self.y * 255.999) as u8;
        let b = (self.z * 255.999) as u8;

        color::Color { r, g, b }
    }
}

pub fn ray_color(ray: Ray) -> color::Color {
    let sphere = Sphere {
        radius: 0.5,
        center: Vec3::new(0.0, 0.0, -1.0),
    };

    let hit = sphere.hit(0.0, 1.0, &ray);

    if let Some(hit) = hit {
        let normal = Vec3::normalized(ray.at(hit.t) - sphere.center);
        let color_norm = (normal + Vec3::from_scalar(1.0)) * 0.5;
        return Vec3::as_color(color_norm);
    }

    let unit_dir = ray.dir;
    let t = 0.5 * (unit_dir.y + 1.0);
    let c0 = Vec3::new(1.0, 1.0, 1.0);
    let c1 = Vec3::new(0.5, 0.7, 1.0);
    Vec3::as_color(lerp(c0, c1, t))
}
