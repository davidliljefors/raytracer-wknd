use core::f32;

use rand::Rng;

use crate::color;
use crate::maths::*;
use crate::hittable::*;


pub fn random_float<R: rand::distributions::uniform::SampleRange<f32>>(range:R) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

pub fn ray_color<T: Hittable>(ray: Ray, world: &T, depth:i32) -> color::Color {
    if depth <= 0 {
        return color::BLACK;
    }

    if let Some(hit) = world.hit(0.01, f32::INFINITY, &ray) {
        let target = hit.point + hit.normal + Vec3::random_in_hemisphere(hit.normal);
        let ray = Ray::new(hit.point, target - hit.point);
        return ray_color(ray, world, depth - 1) * 0.5;
    }

    let unit_dir = ray.dir;
    let t = 0.5 * (unit_dir.y + 1.0);
    let c0 = color::WHITE;
    let c1 = color::Color::new(0.5, 0.7, 1.0);

    color::Color::lerp(c0, c1, t);
    c1
}
