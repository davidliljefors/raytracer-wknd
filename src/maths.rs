use std::ops::*;

use rand::Rng;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

pub fn lerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a * (1.0 - t) + b * t
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}

pub fn clamp<T: PartialOrd>(min:T, max:T, val:T ) -> T {
    if val < min {
        return min;
    }
    if val > max {
        return max;
    }
    val
} 

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn up() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    pub fn right() -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }

    pub fn forward() -> Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }

    pub fn from_scalar(scalar: f32) -> Vec3 {
        Vec3::new(scalar, scalar, scalar)
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn inside_unit_sphere() -> Vec3 {
        let random_vector = || {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            let z = rng.gen_range(-1.0..1.0);
            Vec3{x,y,z}
        };

        loop {
            let test = random_vector();
            if test.length2() < 1.0 {
                return test
            }
        }   
    }

    pub fn on_unit_disc() -> Vec3 {
        let random_vector = || {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            Vec3{x,y,z:0.0}
        };

        loop {
            let test = random_vector();
            if test.length2() < 1.0 {
                return test
            }
        }   
    }

    pub fn reflect(self, normal:Vec3) -> Vec3 {
        self - normal * 2.0 * Vec3::dot(self, normal)
    }

    pub fn refract(self, normal:Vec3, etai_over_etat:f32) -> Vec3 {
        let cos_theta = f32::min(Vec3::dot(-self, normal), 1.0);
        let r_out_perp =  (self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -f32::sqrt(f32::abs(1.0 - r_out_perp.length2()));
        
        r_out_perp + r_out_parallel
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::inside_unit_sphere().normalized()
    }

    pub fn random_in_hemisphere(normal:Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::inside_unit_sphere();
        if Vec3::dot(in_unit_sphere, normal) < 0.0 {
            return -in_unit_sphere;
        }

        in_unit_sphere
    }

    pub fn dot(self: Vec3, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - rhs.y * self.z,
            self.z * rhs.x - rhs.z * self.x,
            self.x * rhs.y - rhs.x * self.y,
        )
    }

    pub fn length2(self: Vec3) -> f32 {
        self.dot(self)
    }

    pub fn length(self: Vec3) -> f32 {
        self.length2().sqrt()
    }

    pub fn normalize(self: &mut Vec3) {
        let one_over_len = 1.0 / self.length();
        *self *= one_over_len;
    }

    pub fn normalized(self: Vec3) -> Vec3 {
        let one_over_len = 1.0 / self.length();
        Vec3 {
            x: self.x * one_over_len,
            y: self.y * one_over_len,
            z: self.z * one_over_len,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => { &self.x },
            1 => { &self.y },
            2 => { &self.z },
            _ => { panic!("Index out of bounds") }
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        let one_over_rhs = 1.0 / rhs;

        self * one_over_rhs
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs
    }
}
