use crate::gamma::gamma_correct;
use core::mem;
use core::ops::Mul;
use micromath::F32Ext as _;
/********************************************* Color Part ***********************************************************/
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

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
        let g: f32 = (self.g as f32 * rhs).clamp(0.0, 255.0).round();
        let b: f32 = (self.b as f32 * rhs).clamp(0.0, 255.0).round();
        let r: f32 = (self.r as f32 * rhs).clamp(0.0, 255.0).round();
        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
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
/************************************************* Image Part *****************************************************/
#[repr(transparent)]
pub struct Image([Color; 64]);

impl Image {
    pub fn new_solid(color: Color) -> Self {
        Image([color; 64])
    }
    pub fn row(&self, row: usize) -> &[Color] {
        if row > 7 {
            panic!("invalid row U Monster")
        }
        &self.0[row*8..(row + 1) * 8]
    }
    pub fn gradient(color: Color) -> Self {
        let mut im: Image = Default::default();

        for row in 0..=7 {
            for col in 0..=7 {
                let nb: f32 = 1.0 + (row * row + col) as f32;
                im[(row, col)] = color / nb;
            }
        }
        im
    }
}

impl Default for Image {
    fn default() -> Self {
        let color: Color = Default::default();
        Image([color; 64])
    }
}

impl core::ops::Index<(usize, usize)> for Image {
    type Output = Color;

    // Required method
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 > 7 || index.1 > 7 {
            panic!("invalide row or column U Monster")
        }
        &self.0[index.0 * 8 + index.1]
    }
}

impl core::ops::IndexMut<(usize, usize)> for Image {
    // Required method
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 > 7 || index.1 > 7 {
            panic!("invalide row or column U Monster")
        }
        &mut self.0[index.0 * 8 + index.1]
    }
}

impl AsRef<[u8; 192]> for Image {
    fn as_ref(&self) -> &[u8; 192] {
        unsafe { mem::transmute::<&[Color; 64], &[u8; 192]>(&self.0) }
    }
}

impl AsMut<[u8; 192]> for Image {
    fn as_mut(&mut self) -> &mut [u8; 192] {
        unsafe { mem::transmute::<&mut [Color; 64], &mut [u8; 192]>(&mut self.0) }
    }
}
