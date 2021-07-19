#![allow(dead_code)]
use core::f32;

use crate::helpers::{random_float, ray_color};
use crate::hittable::HittableList;
use crate::color::Color;

mod camera;
mod material;
mod color;
mod helpers;
mod hittable;
mod maths;

fn main() {
    // Image
    let filename = std::ffi::CString::new("image.bmp").unwrap();

    let camera = camera::Camera::create();

    let samples_per_pixel = 200;
    let depth = 50;
    let width = 720;
    let height = (width as f32 / camera.aspect()) as i32;
    let mut image = Vec::<u8>::new();

    // World
    let mut world = HittableList::new();
    //let sphere = hittable::Sphere::new(0.0, 0.0, -1.0, 0.5);
    let center_mat = material::Lambertian::create(Color{r:0.8, g:0.6, b:0.0});
    let ground_mat = material::Lambertian::create(Color{r:0.2, g:0.8, b:0.3});
    world.add(Box::new(hittable::Sphere::new(0.0, 0.0, -1.0, 0.5, center_mat)));
    world.add(Box::new(hittable::Sphere::new(0.0, -100.5, -1.0, 100.0, ground_mat)));

    for y in (0..height).rev() {
        print!("\x1B[2J\x1B[1;1H"); // Clear and reset console print

        println!(
            "Working {:.0}%",
            ((height - y) as f64 / height as f64) * 100.0
        );
        for x in 0..width {
            let mut accum_color = color::BLACK;
            for _ in 0..samples_per_pixel {
                let ru = random_float(0.0..1.0);
                let rv = random_float(0.0..1.0);
                let v = (y as f32 + rv) / (height as f32 - 1.0);
                let u = (x as f32 + ru) / (width as f32 - 1.0);
                accum_color += ray_color(camera.get_ray(u, v), &world, depth);
            }
            let accum_color = accum_color.as_rgb8(samples_per_pixel);
            image.push(accum_color.r);
            image.push(accum_color.g);
            image.push(accum_color.b);
        }
    }
    println!("Writing output to {:?}", filename);
    stb::image_write::stbi_write_bmp(&filename, width, height, 3, &image);
    println!("Done!");
}
