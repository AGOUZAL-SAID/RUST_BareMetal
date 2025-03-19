use crate::gamma::gamma_correct;
use core::ops::Mul;


#[derive(Copy, Clone, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

pub fn floor(nb: f32) -> u32 {
    let mut i: f32 = 0 as f32;
    let mut entier: u32 = 0;
    loop {
        if i > nb {
            if entier == 0 {
                return 0;
            } else {
                return entier - 1;
            }
        }
        i += 1.0;
        entier += 1;
    }
}



impl Color {
    pub fn gamma_correct(&self) -> Self {
        let mut my_color = Color {
            r: self.r,
            g: self.g,
            b: self.b,
        };
        my_color.r = gamma_correct(my_color.r);
        my_color.g = gamma_correct(my_color.g);
        my_color.b = gamma_correct(my_color.b);
        my_color
    }
}


impl core::ops::Mul<f32> for Color {
    type Output = Color;
    // Required method
    fn mul(self, rhs: f32) -> Self::Output {
        let g: u32 = self.g as u32 * floor(rhs);
        let b: u32 = self.g as u32 * floor(rhs);
        let r: u32 = self.g as u32 * floor(rhs);
        Color {
            r: (r) as u8,
            g: (g) as u8,
            b: (b) as u8,
        }
    }
}


impl core::ops::Div<f32> for Color {
    type Output = Color;
    // Required method
    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Don't divide by zero U Monster")
        }
        self.mul(1_f32 / rhs)
    }
}
pub struct Image;
