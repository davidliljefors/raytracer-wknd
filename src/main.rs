#![allow(dead_code)]
use core::{f32};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::color::Color;
use crate::helpers::{random_float, ray_color};
use crate::hittable::HittableList;

mod camera;
mod color;
mod helpers;
mod hittable;
mod material;
mod maths;


fn main() {
    // Image
    let filename = std::ffi::CString::new("image.bmp").unwrap();

    let camera = camera::Camera::create();

    let mut num_threads = 1;
    let mut samples_per_pixel = 10;
    let mut width = 600;
    let mut depth = 20;
    
    for arg in std::env::args() {
        let mut args = arg.split('=');
        let command = args.next().expect("invalid args");

        if let Some(value) = args.next() {
            let value = value.parse::<i32>().expect("invalid number");
            match &command[..] {
                "-t" => {num_threads = value},
                "-s" => {samples_per_pixel = value},
                "-w" => {width = value},
                "-d" => {depth = value},
                _ => {}
            }
        }
    }

    let height = (width as f32 / camera.aspect()) as i32;

    // World
    let mut world = HittableList::new();
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
    world.add(hittable::Sphere::create(-1.0, 0.0, -1.0, 0.35, aluminium.clone()));
    world.add(hittable::Sphere::create(1.5, 0.0, -2.0, 0.35, gold.clone()));
    world.add(hittable::Sphere::create(1.8, 0.4, -2.0, 0.45, lambert_blue.clone()));
    world.add(hittable::Sphere::create(0.0, 0.0, -1.0, 0.5, glass.clone()));
    world.add(hittable::Sphere::create(1.0, -0.3, -1.0, 0.2, glass.clone()));
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
    
    let all_images = Arc::new( Mutex::new( Vec::<Color>::new() ) );
    all_images.lock().unwrap().resize((width * height) as usize, color::BLACK);
    
    let samples_per_thread = samples_per_pixel;

    let thread_images = all_images.clone();
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
                    accum_color += ray_color(camera.get_ray(u, v), &world, depth);
                }
                thread_result.push(accum_color);
            }
        }
        
        let mut all = thread_images.lock().unwrap();

        for (total, local) in all.iter_mut().zip(thread_result.iter()) {
            *total += *local;
        }
        
    };

    let mut threads = Vec::new();
    let process_image = std::sync::Arc::new( process_image );
    for _ in 0..num_threads {

        let p1 = process_image.clone();
        let t1 = std::thread::spawn( move || p1.deref()() );
        threads.push( t1 );
    }

    for t in threads {
        t.join().unwrap();
    }
    
    let mut image = Vec::<u8>::new();

    for color in all_images.lock().unwrap().iter() {
        let rgb8 = color.as_rgb8(samples_per_pixel * num_threads);
        image.push(rgb8.r);
        image.push(rgb8.g);
        image.push(rgb8.b);
    }

    println!("Used {} threads", num_threads);
    println!("Used {}*{} Samples", samples_per_pixel, num_threads);
    println!("Image size {}x{}", width, height);
    println!("Writing output to {:?}", filename);
    stb::image_write::stbi_write_bmp(&filename, width, height, 3, &image);
    println!("Done!");
}
