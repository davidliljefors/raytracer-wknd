use crate::color::Color;
use crate::helpers::random_float;
use crate::hittable::HitRecord;
use crate::maths::{Ray, Vec3};

pub type MaterialPtr = std::sync::Arc<dyn Material>;

pub trait Material: Sync + Send {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn create(albedo: Color) -> MaterialPtr {
        std::sync::Arc::new(Lambertian { albedo })
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

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn create(albedo: Color, fuzz: f32) -> MaterialPtr {
        let mut fuzz = fuzz;
        if fuzz > 1.0 {
            fuzz = 1.0;
        }

        std::sync::Arc::new(Metal { albedo, fuzz })
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(ray.dir.normalized(), hit.normal);

        let scattered = Ray::new(
            hit.point,
            reflected + Vec3::inside_unit_sphere() * self.fuzz,
        );
        if Vec3::dot(scattered.dir, hit.normal) > 0.0 {
            return Some((self.albedo, scattered));
        }

        None
    }
}

pub struct Dieletric {
    albedo: Color,
    index_of_refraction: f32,
}

impl Dieletric {
    pub fn create(albedo: Color, index_of_refraction: f32) -> MaterialPtr {
        std::sync::Arc::new(Dieletric {
            albedo,
            index_of_refraction,
        })
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
    }
}

impl Material for Dieletric {
    fn scatter(&self, ray: Ray, hit: HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_dir = ray.dir.normalized();
        let cos_theta = f32::min(Vec3::dot(-unit_dir, hit.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dieletric::reflectance(cos_theta, refraction_ratio) > random_float(0.0..1.0)
        {
            unit_dir.reflect(hit.normal)
        } else {
            unit_dir.refract(hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.point, direction);

        Some((self.albedo, scattered))
    }
}
