use core::f32;

use rand::Rng;

use crate::color;
use crate::maths::*;
use crate::hittable::*;


impl Vec3 {
    pub fn as_color(self: Vec3, samples_per_pixel:i32) -> color::Color {
        let scale = 1.0 / samples_per_pixel as f32;

        let scaled = self * scale;

        let r = (256.0 * clamp(0.0, 0.9999, scaled.x).sqrt()) as u8;
        let g = (256.0 * clamp(0.0, 0.9999, scaled.y).sqrt()) as u8;
        let b = (256.0 * clamp(0.0, 0.9999, scaled.z).sqrt()) as u8;
        color::Color { r, g, b }
    }
}

pub fn random_float<R: rand::distributions::uniform::SampleRange<f32>>(range:R) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

pub fn ray_color<T: Hittable>(ray: Ray, world: &T, depth:i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some(hit) = world.hit(0.01, f32::INFINITY, &ray) {
        let target = hit.point + hit.normal + Vec3::random_in_hemisphere(hit.normal);
        let ray = Ray::new(hit.point, target - hit.point);
        return ray_color(ray, world, depth - 1) * 0.5;
    }

    let unit_dir = ray.dir;
    let t = 0.5 * (unit_dir.y + 1.0);
    let c0 = Vec3::new(1.0, 1.0, 1.0);
    let c1 = Vec3::new(0.5, 0.7, 1.0);

    lerp(c0, c1, t)
}
