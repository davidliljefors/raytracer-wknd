use core::f32;

use rand::Rng;

use crate::color;
use crate::hittable::*;
use crate::maths::*;

pub fn random_float<R: rand::distributions::uniform::SampleRange<f32>>(range: R) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

pub fn ray_color<T: Hittable>(ray: Ray, world: &T, background:color::Color, depth: i32) -> color::Color {
    if depth <= 0 {
        return color::BLACK;
    }

    if let Some(hit) = world.hit(0.01, f32::INFINITY, &ray) {
        let mat = &hit.material;
        let emitted = hit.material.emitted();

        if let Some((attenuation, scattered)) = mat.scatter(ray, &hit) {
            return emitted + attenuation * ray_color(scattered, world, background, depth - 1);
        }
        return emitted
    }


    background
    // let unit_dir = ray.dir;
    // let t = 0.5 * (unit_dir.y + 1.0);
    // let c0 = color::WHITE;
    // let c1 = color::Color::new(0.5, 0.7, 1.0);

    // color::Color::lerp(c0, c1, t)
}

pub fn hit_albedo<T: Hittable>(ray: Ray, world: &T) -> color::Color {

    if let Some(hit) = world.hit(0.01, f32::INFINITY, &ray) {
        return hit.material.albedo();
    }

    let unit_dir = ray.dir;
    let t = 0.5 * (unit_dir.y + 1.0);
    let c0 = color::WHITE;
    let c1 = color::Color::new(0.5, 0.7, 1.0);

    color::Color::lerp(c0, c1, t)
}

pub fn hit_normal<T: Hittable>(ray: Ray, world: &T) -> color::Color {
    if let Some(hit) = world.hit(0.01, f32::INFINITY, &ray) {
        return color::Color::from_vec3(hit.normal);
    }

    color::BLACK
}

pub fn write_image_flipped(name: &str, image: &[color::Color], width: i32, height: i32) {
    let mut image_rgb8 = Vec::<u8>::new();

    for color in image.iter().rev() {
        let rgb8 = color.as_rgb8();
        image_rgb8.push(rgb8.r);
        image_rgb8.push(rgb8.g);
        image_rgb8.push(rgb8.b);
    }

    println!("Saved file {}", name);
    let filename = std::ffi::CString::new(name).unwrap();

    stb::image_write::stbi_write_png(&filename, width, height, 3, &image_rgb8, 0);
}
