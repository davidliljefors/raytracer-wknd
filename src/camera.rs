use crate::maths::Ray;
use crate::maths::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    aspect_ratio: f32,
}

impl Camera {
    pub fn create() -> Camera {
        let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_h = 2.0;
        let viewport_w = aspect_ratio * viewport_h;
        let focal_length: f32 = 1.0;

        let origin = Vec3::zero();
        let horizontal = Vec3::new(viewport_w, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_h, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            aspect_ratio,
        }
    }

    pub fn aspect(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
