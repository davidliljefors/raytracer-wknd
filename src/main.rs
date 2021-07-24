#![allow(dead_code)]
use core::f32;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::color::Color;
use crate::helpers::{random_float, ray_color};
use crate::hittable::HittableList;
use crate::maths::Vec3;
use crate::bvh::Bvh;

type TsImage = Arc<Mutex<Vec<Color>>>;

mod camera;
mod aabb;
mod color;
mod helpers;
mod hittable;
mod material;
mod maths;
mod bvh;

fn make_world() -> Arc<Bvh> {
    let mut world =HittableList::new();
    //let sphere = hittable::Sphere::new(0.0, 0.0, -1.0, 0.5);
    let lambert_red = material::Lambertian::create(Color {
        r: 0.93,
        g: 0.4,
        b: 0.1,
    });
    let lambert_blue = material::Lambertian::create(Color {
        r: 0.03,
        g: 0.34,
        b: 0.93,
    });
    let aluminium = material::Metal::create(
        Color {
            r: 0.8,
            g: 0.8,
            b: 0.8,
        },
        0.05,
    );
    let gold = material::Metal::create(
        Color {
            r: 1.0,
            g: 0.95,
            b: 0.55,
        },
        0.11,
    );
    let glass = material::Dieletric::create(
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
        1.5,
    );
    let ground_mat = material::Lambertian::create(Color {
        r: 0.2,
        g: 0.8,
        b: 0.3,
    });
    world.add(hittable::Sphere::create(
        -1.0,
        0.0,
        -1.0,
        0.35,
        aluminium.clone(),
    ));
    world.add(hittable::Sphere::create(1.5, 0.0, -2.0, 0.35, gold.clone()));
    world.add(hittable::Sphere::create(
        1.8,
        0.4,
        -2.0,
        0.45,
        lambert_blue.clone(),
    ));
    world.add(hittable::Sphere::create(0.0, 0.0, -1.0, 0.5, glass.clone()));
    world.add(hittable::Sphere::create(
        1.0,
        -0.3,
        -1.0,
        0.2,
        glass.clone(),
    ));
    world.add(hittable::Sphere::create(
        0.0,
        0.0,
        -1.0,
        -0.49,
        glass.clone(),
    ));
    world.add(hittable::Sphere::create(0.0, 0.0, -3.0, 0.5, lambert_red));
    world.add(hittable::Sphere::create(
        0.0, -100.5, -3.0, 100.0, ground_mat,
    ));

    std::sync::Arc::new(Bvh::new(world))
}

fn main() {
    // Image
    let filename = std::ffi::CString::new("image.bmp").unwrap();

    let mut num_threads = 6;
    let mut samples_per_pixel = 60;
    let mut width = 900;
    let mut depth = 20;

    for arg in std::env::args() {
        let mut args = arg.split('=');
        let command = args.next().expect("invalid args");

        if let Some(value) = args.next() {
            let value = value.parse::<i32>().expect("invalid number");
            match &command[..] {
                "-t" => num_threads = value,
                "-s" => samples_per_pixel = value,
                "-w" => width = value,
                "-d" => depth = value,
                _ => {}
            }
        }
    }

    let camera_pos = Vec3::new(2.0,1.0, 1.0);
    let camera_focus = Vec3::new(1., -0.3, -1.0);
    let focus_dist = (camera_pos-camera_focus).length();

    let camera = camera::Camera::create(
        camera_pos,
        camera_focus,
        Vec3::up(),
        16.0 / 9.0,
        30.0,
        0.1,
        focus_dist,
    );  
    let height = (width as f32 / camera.aspect()) as i32;

    // World
    let world = make_world();

    let image = Arc::new(Mutex::new(Vec::<Color>::new()));
    image
        .lock()
        .unwrap()
        .resize((width * height) as usize, color::BLACK);

    let samples_per_thread = (samples_per_pixel as f32 / num_threads as f32).ceil() as i32;

    let closure_image = image.clone();
    let process_image = move || {
        let mut thread_result = Vec::<Color>::new();
        for y in (0..height).rev() {
            print!("\x1B[2J\x1B[1;1H"); // Clear and reset console print

            println!(
                "Working {:.0}%",
                ((height - y) as f64 / height as f64) * 100.0
            );
            for x in 0..width {
                let mut accum_color = color::BLACK;
                for _ in 0..samples_per_thread {
                    let ru = random_float(0.0..1.0);
                    let rv = random_float(0.0..1.0);
                    let v = (y as f32 + rv) / (height as f32 - 1.0);
                    let u = (x as f32 + ru) / (width as f32 - 1.0);
                    accum_color += ray_color(camera.get_ray(u, v), world.deref(), depth);
                }
                thread_result.push(accum_color);
            }
        }

        let mut thread_local_canvas = closure_image.lock().unwrap();

        for (pixel, local) in thread_local_canvas.iter_mut().zip(thread_result.iter()) {
            *pixel += *local;
        }
    };

    let mut threads = Vec::new();
    let time_before_loop = std::time::Instant::now();
    let process_image = std::sync::Arc::new(process_image);
    for _ in 0..num_threads {
        let p1 = process_image.clone();
        let t1 = std::thread::spawn(move || p1.deref()());
        threads.push(t1);
    }

    for t in threads {
        t.join().unwrap();
    }

    let loop_dur = std::time::Instant::now() - time_before_loop;
    let mut image_rgb8 = Vec::<u8>::new();

    for color in image.lock().unwrap().iter() {
        let rgb8 = color.as_rgb8(samples_per_thread * num_threads);
        image_rgb8.push(rgb8.r);
        image_rgb8.push(rgb8.g);
        image_rgb8.push(rgb8.b);
    }
    println!("Render took {} seconds", loop_dur.as_secs_f64());
    println!("Used {} threads", num_threads);
    println!("Used {}*{} Samples", samples_per_thread, num_threads);
    println!("Image size {}x{}", width, height);
    println!("Writing output to {:?}", filename);
    stb::image_write::stbi_write_bmp(&filename, width, height, 3, &image_rgb8);
    println!("Done!");
}
