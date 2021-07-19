use crate::maths::{Vec3, clamp};

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub const BLACK : Color = Color{r:0.0, g:0.0, b:0.0};
pub const WHITE : Color = Color{r:1.0, g:1.0, b:1.0};

pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_vec3( v:Vec3 ) -> Color {
        Color{ r:v.x, g:v.y, b:v.z }
    } 

    pub fn new(r:f32, g:f32, b:f32) -> Color {
        Color{r, g, b}
    }

    pub fn as_rgb8(self: Color, samples_per_pixel:i32 ) -> RGB8 {
        let scale = 1.0 / samples_per_pixel as f32;

        let scaled = self * scale;
        
        let r = (256.0 * clamp(0.0, 0.9999, scaled.r).sqrt()) as u8;
        let g = (256.0 * clamp(0.0, 0.9999, scaled.g).sqrt()) as u8;
        let b = (256.0 * clamp(0.0, 0.9999, scaled.b).sqrt()) as u8;
        RGB8 { r, g, b }
    }

    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        a * (1.0 - t) + b * t
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color{ r:self.r*rhs.r, g:self.g*rhs.g, b:self.b*rhs.b }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color{ r:self.r*rhs, g:self.g*rhs, b:self.b*rhs }
    }
}