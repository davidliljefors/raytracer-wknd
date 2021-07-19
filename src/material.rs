use crate::maths::{Ray, Vec3};
use crate::hittable::HitRecord;
use crate::color::Color;

pub type MaterialPtr = std::rc::Rc<dyn Material>;

trait Material {
    fn scatter(ray:Ray, hit:HitRecord) -> Option<(i32, i32)>;
}