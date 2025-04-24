use crate::{Color, Image};
use core::convert::Infallible;
use core::ops::IndexMut;
use embedded_graphics::Pixel;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::prelude::Size;
use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
impl From<Rgb888> for Color {
    fn from(value: Rgb888) -> Self {
        Color {
            r: value.r(),
            g: value.g(),
            b: value.b(),
        }
    }
}

impl DrawTarget for Image {
    type Color = Rgb888;
    type Error = Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, c) in pixels {
            let x = coord.x;
            let y = coord.y;
            // only draw within 0..8 bounds
            if (0..8).contains(&x) && (0..8).contains(&y) {
                // convert Rgb888 into our Color
                *self.index_mut((x as usize, y as usize)) = c.into();
            }
        }
        Ok(())
    }
}

impl OriginDimensions for Image {
    fn size(&self) -> Size {
        Size::new(8, 8)
    }
}
