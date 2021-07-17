use crate::color;
use crate::maths::*;

impl Vec3 {
    pub fn as_color(self:Vec3) -> color::Color {
        let r = ( self.x * 255.999 ) as u8;
        let g = ( self.y * 255.999 ) as u8;
        let b = ( self.z * 255.999 ) as u8;

        color::Color{ r, g, b }
    }
}

pub fn hit_sphere(center: Vec3, radius: f32, ray: Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.dir.length2();
    let half_b = oc.dot(ray.dir);
    let c = oc.length2() - radius * radius;
    let discriminant = half_b * half_b - a*c;
    if discriminant < 0.0 {
        return -1.0;
    }
    -half_b - discriminant.sqrt() / a
}

pub fn ray_color(ray: Ray) -> color::Color {
    let sphere_pos = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(sphere_pos, 0.5, ray);
    
    if t > 0.0 {
        let normal = Vec3::normalized(ray.at(t) - sphere_pos);
        let color_norm = (normal + Vec3::from_scalar(1.0)) * 0.5;
        return Vec3::as_color(color_norm);
    }

    let unit_dir = ray.dir;
    let t = 0.5 * (unit_dir.y + 1.0);
    let c0 = Vec3::new(1.0, 1.0, 1.0);
    let c1 = Vec3::new(0.5, 0.7, 1.0);
    Vec3::as_color(lerp(c0, c1, t))
}