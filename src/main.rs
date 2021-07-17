use core::f32;

use crate::helpers::ray_color;

mod color;
mod helpers;
mod maths;

fn main() {
    let filename = std::ffi::CString::new("image.bmp").unwrap();
    let aspect: f32 = 16.0 / 9.0;
    let width = 1600;
    let height = (width as f32 / aspect) as i32;

    let viewport_h = 2.0;
    let viewport_w = aspect * viewport_h;
    let focal_length: f32 = 1.0;

    let origin = maths::Vec3::zero();
    let horizontal = maths::Vec3::new(viewport_w, 0.0, 0.0);
    let vertical = maths::Vec3::new(0.0, viewport_h, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - maths::Vec3::new(0.0, 0.0, focal_length);

    let mut image = Vec::<u8>::new();

    for y in (0..height).rev() {
        print!("\x1B[2J\x1B[1;1H"); // Clear and reset console print

        println!("Working {:.0}%", ((height-y) as f64 / height as f64) * 100.0);
        for x in 0..width {
            let u = x as f32 / (width as f32 - 1.0);
            let v = y as f32 / (height as f32 - 1.0);

            let ray = maths::Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin
            );
            let color = ray_color(ray);
            image.push(color.r);
            image.push(color.g);
            image.push(color.b);
        }
    }
    println!("Writing output to {:?}", filename);
    stb::image_write::stbi_write_bmp(&filename, width, height, 3, &image);
    println!("Done!");
}
