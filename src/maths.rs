use std::ops::*;

#[allow(dead_code)]

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

pub fn lerp(a: Vec3, b: Vec3, t:f32) -> Vec3 {   
    a * (1.0-t) + b * t
}

impl Ray {
    pub fn new(origin:Vec3, dir:Vec3) -> Ray {
        Ray{ origin, dir }
    }

    pub fn at(&self, t:f32) -> Vec3 {
        self.origin + self.dir * t
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn from_scalar(scalar:f32) -> Vec3 {
        Vec3::new(scalar, scalar, scalar)
    }

    pub fn zero() -> Vec3 {
        Vec3{ x:0.0, y:0.0, z:0.0 }
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
