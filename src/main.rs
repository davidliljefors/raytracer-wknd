#![allow(dead_code)]
use core::f32;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::bvh::Bvh;
use crate::color::Color;
use crate::helpers::*;
use crate::hittable::HittableList;
use crate::maths::Vec3;

type TsImage = Arc<Mutex<Vec<Color>>>;

mod aabb;
mod bvh;
mod camera;
mod color;
mod helpers;
mod hittable;
mod material;
mod maths;

fn make_world() -> Arc<Bvh> {
    let mut world = HittableList::new();
    //let sphere = hittable::Sphere::new(0.0, 0.0, -1.0, 0.5);
    let lambert_red = material::Lambertian::create(Color {
        r: 0.93,
        g: 0.0,
        b: 0.0,
    });
    let sun = material::DiffuseLight::create(Color {
        r: 1.00,
        g: 1.00,
        b: 0.90,
    });
    let lamp = material::DiffuseLight::create(Color {
        r: 1.00,
        g: 1.00,
        b: 0.0,
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
    world.add(hittable::Sphere::create(
        2.0,
        2.0,
        -2.0,
        0.5,
        lamp
    ));
    world.add(hittable::Sphere::create(1.5, 0.0, -2.0, 0.35, gold.clone()));
    world.add(hittable::Sphere::create(
        -80.0,
        60.0,
        -50.0,
        70.0,
        sun.clone(),
    ));
    world.add(hittable::Sphere::create(0.0, 0.0, -1.0, 0.5, glass.clone()));
    world.add(hittable::Sphere::create(
        1.0,
        -0.3,
        -1.0,
        0.2,
        glass.clone(),
    ));
    world.add(hittable::Sphere::create(0.0, 0.0, -3.0, 0.5, lambert_red));
    world.add(hittable::Sphere::create(
        0.0, -100.5, -3.0, 100.0, ground_mat,
    ));

    std::sync::Arc::new(Bvh::new(world))
}

fn collect_normals<T: hittable::Hittable>(
    world: &T,
    camera: camera::Camera,
    width: i32,
    height: i32,
) -> Vec<Color> {
    let mut normals = Vec::<Color>::new();
    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / (width as f32 - 1.0);
            let v = y as f32 / (height as f32 - 1.0);
            normals.push(hit_normal(camera.straight_ray(u, v), world.deref()));
        }
    }
    normals
}

fn collect_albedo<T: hittable::Hittable>(
    world: &T,
    camera: camera::Camera,
    width: i32,
    height: i32,
) -> Vec<Color> {
    let mut normals = Vec::<Color>::new();
    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / (width as f32 - 1.0);
            let v = y as f32 / (height as f32 - 1.0);
            normals.push(hit_albedo(camera.straight_ray(u, v), world.deref()));
        }
    }
    normals
}

fn main() {
    // Image
    const bg : color::Color = color::Color{r:0.0001, g:0.0002, b:0.002};
    let mut num_threads = 10;
    let mut samples_per_pixel = 400;
    let mut width = 2000;
    let mut depth = 100;

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

    let world = make_world();

    let camera_pos = Vec3::new(5.0, 2.5, 3.0);
    let camera_focus = Vec3::new(1., -0.3, -1.0);
    let focus_dist = (camera_pos - camera_focus).length();

    let camera = camera::Camera::create(
        camera_pos,
        camera_focus,
        Vec3::up(),
        16.0 / 9.0,
        30.0,
        0.0,
        focus_dist,
    );
    let height = (width as f32 / camera.aspect()) as i32;
    // World

    let image = Arc::new(Mutex::new(Vec::<Color>::new()));
    image
        .lock()
        .unwrap()
        .resize((width * height) as usize, color::BLACK);

    let closure_image = image.clone();
    let process_image = move |begin, end, world: Arc<Bvh>| {
        let mut thread_result = Vec::<Color>::new();
        let scale = 1.0 / samples_per_pixel as f32;
        for y in begin..end {
            for x in 0..width {
                let mut accum_color = color::BLACK;
                for _ in 0..samples_per_pixel {
                    let ru = random_float(0.0..1.0);
                    let rv = random_float(0.0..1.0);
                    let v = (y as f32 + rv) / (height as f32 - 1.0);
                    let u = (x as f32 + ru) / (width as f32 - 1.0);
                    accum_color += ray_color(camera.get_ray(u, v), world.deref(), bg, depth);
                }
                thread_result.push(accum_color * scale);
            }
        }

        let mut thread_local_canvas = closure_image.lock().unwrap();
        let offset = begin * width;
        for (index, color) in thread_result.iter().enumerate() {
            thread_local_canvas[index + offset as usize] += *color;
        }
    };

    let mut threads = Vec::new();
    let time_before_loop = std::time::Instant::now();
    let process_image = std::sync::Arc::new(process_image);

    let subrange_step = height / num_threads;
    let remainder = height % num_threads;
    assert!(
        subrange_step > 0,
        "dont use more threads than image height :("
    );

    for thread_num in 0..num_threads {
        let range_start = thread_num * subrange_step;
        let mut range_end = (thread_num + 1) * subrange_step;
        if thread_num == num_threads - 1 {
            range_end += remainder;
        }
        let w = world.clone();
        let p1 = process_image.clone();
        let t1 = std::thread::spawn(move || p1.deref()(range_start, range_end, w));
        threads.push(t1);
    }

    for t in threads {
        t.join().unwrap();
    }

    let loop_dur = std::time::Instant::now() - time_before_loop;
    let normal_data = collect_normals(world.deref(), camera, width, height);
    let albedo_data = collect_albedo(world.deref(), camera, width, height);
    write_image_flipped("beauty.png", &image.lock().unwrap(), width, height);
    write_image_flipped("normal.png", &normal_data, width, height);
    write_image_flipped("albedo.png", &albedo_data, width, height);

    println!("Render took {} seconds", loop_dur.as_secs_f64());
    println!("Used {} threads", num_threads);
    println!("Used {} Samples", samples_per_pixel);
    println!("Image size {}x{}", width, height);
    println!("Done!");
}
