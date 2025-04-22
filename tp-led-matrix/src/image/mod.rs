use crate::gamma::gamma_correct;
use core::mem;
use core::ops::Mul;
use micromath::F32Ext as _;

/********************************************* Color Part ***********************************************************/
/// Represents an RGB color with 8-bit channels.
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// Predefined basic colors
pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };

impl Color {
    /// Apply gamma correction to each channel via external lookup.
    pub fn gamma_correct(&self) -> Self {
        let mut my_color = *self;
        my_color.r = gamma_correct(my_color.r);
        my_color.g = gamma_correct(my_color.g);
        my_color.b = gamma_correct(my_color.b);
        my_color
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    /// Scale each channel by `rhs`, clamping to [0,255] and rounding.
    fn mul(self, rhs: f32) -> Self::Output {
        let r = (self.r as f32 * rhs).clamp(0.0, 255.0).round();
        let g = (self.g as f32 * rhs).clamp(0.0, 255.0).round();
        let b = (self.b as f32 * rhs).clamp(0.0, 255.0).round();
        Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
        }
    }
}

impl core::ops::Div<f32> for Color {
    type Output = Color;

    /// Divide each channel by `rhs`, panicking on division by zero.
    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0.0 {
            panic!("Don't divide by zero U Monster");
        }
        self.mul(1.0 / rhs)
    }
}

/************************************************* Image Part *****************************************************/
/// A transparent wrapper around a flat 8×8 array of `Color`.
#[repr(transparent)]
pub struct Image([Color; 64]);

impl Image {
    /// Create an `Image` from a raw 8×8 buffer.
    pub fn new_im(buffer: [Color; 64]) -> Self {
        Image(buffer)
    }

    /// Create a solid-color image using a `const fn`.
    pub const fn new_solid(color: Color) -> Self {
        Image([color; 64])
    }

    /// Borrow a single row (0–7) as a slice of 8 `Color`s.
    pub fn row(&self, row: usize) -> &[Color] {
        if row > 7 {
            panic!("invalid row U Monster");
        }
        &self.0[row * 8..(row + 1) * 8]
    }

    /// Generate a simple gradient image: color scaled by 1 + (row² + col).
    pub fn gradient(color: Color) -> Self {
        let mut im: Image = Default::default();
        for row in 0..8 {
            for col in 0..8 {
                let factor: f32 = 1.0 + (row * row + col) as f32;
                im[(row, col)] = color / factor;
            }
        }
        im
    }
}

impl Default for Image {
    /// Default image is all black.
    fn default() -> Self {
        Image([Color::default(); 64])
    }
}

impl core::ops::Index<(usize, usize)> for Image {
    type Output = Color;

    /// Index via (row, col), with bounds checking.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (r, c) = index;
        if r > 7 || c > 7 {
            panic!("invalid row or column U Monster");
        }
        &self.0[r * 8 + c]
    }
}

impl core::ops::IndexMut<(usize, usize)> for Image {
    /// Mutable indexing via (row, col), with bounds checking.
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (r, c) = index;
        if r > 7 || c > 7 {
            panic!("invalid row or column U Monster");
        }
        &mut self.0[r * 8 + c]
    }
}

impl AsRef<[u8; 192]> for Image {
    /// View the image buffer as raw bytes (3 bytes per pixel).
    fn as_ref(&self) -> &[u8; 192] {
        unsafe { mem::transmute::<&[Color; 64], &[u8; 192]>(&self.0) }
    }
}

impl AsMut<[u8; 192]> for Image {
    /// Mutably view the image buffer as raw bytes.
    fn as_mut(&mut self) -> &mut [u8; 192] {
        unsafe { mem::transmute::<&mut [Color; 64], &mut [u8; 192]>(&mut self.0) }
    }
}
