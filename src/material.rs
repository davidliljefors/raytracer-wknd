use crate::color::Color;
use crate::hittable::HitRecord;
use crate::maths::{Ray, Vec3};

pub type MaterialPtr = std::rc::Rc<dyn Material>;

pub trait Material {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn create(albedo: Color) -> MaterialPtr {
        std::rc::Rc::new(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, hit: HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        if scatter_direction.length2() < f32::EPSILON {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}
