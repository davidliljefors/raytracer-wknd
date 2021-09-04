use crate::maths::Ray;
use crate::maths::Vec3;

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    aspect_ratio: f32,
    lens_radius: f32,
}

impl Camera {
    pub fn create(
        from: Vec3,
        to: Vec3,
        up: Vec3,
        aspect_ratio: f32,
        vfov: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let h = f32::tan(f32::to_radians(vfov));
        let viewport_h = 2.0 * h;
        let viewport_w = aspect_ratio * viewport_h;

        let w = Vec3::normalized(from - to);
        let u = Vec3::normalized(Vec3::cross(up, w));
        let v = Vec3::cross(w, u);

        let origin = from;
        let horizontal = u * viewport_w * focus_dist;
        let vertical = v * viewport_h * focus_dist;
        let lower_left_corner = origin - horizontal * 0.5 - vertical * 0.5 - w * focus_dist;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            aspect_ratio,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn aspect(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = Vec3::on_unit_disc() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
        )
    }

    pub fn straight_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        )
    }
}
