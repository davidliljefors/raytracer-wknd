pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}
