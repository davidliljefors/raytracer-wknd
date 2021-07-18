use crate::color;
use crate::maths::*;
use crate::hittable::*;

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

    if let Some(hit) = sphere.hit(0.0, 1.0, &ray) {
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
