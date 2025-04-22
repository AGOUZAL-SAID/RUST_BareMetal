use crate::{Color, Image};
use embassy_stm32::{
    gpio::*,
    peripherals::{PA2, PA3, PA4, PA5, PA6, PA7, PA15, PB0, PB1, PB2, PC3, PC4, PC5},
};
use embassy_time::{Ticker, Timer};

/// Represents an 8×8 LED matrix driven by shift‑register control signals.
pub struct Matrix<'a> {
    sb: Output<'a>,
    lat: Output<'a>,
    rst: Output<'a>,
    sck: Output<'a>,
    sda: Output<'a>,
    rows: [Output<'a>; 8],
}

impl Matrix<'static> {
    /// Create and initialize the matrix:
    /// - Configure each pin at VeryHigh speed
    /// - Drive RST low, then high after 100 ms
    /// - Set SB and LAT high by default
    /// - Initialize bank 0 (144 one bits) via `init_bank0()`
    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        pa2: PA2,
        pa3: PA3,
        pa4: PA4,
        pa5: PA5,
        pa6: PA6,
        pa7: PA7,
        pa15: PA15,
        pb0: PB0,
        pb1: PB1,
        pb2: PB2,
        pc3: PC3,
        pc4: PC4,
        pc5: PC5,
    ) -> Self {
        // Configure control pins with initial levels
        let pin_sb = Output::new(pc5, Level::Low, Speed::VeryHigh);
        let pin_lat = Output::new(pc4, Level::Low, Speed::VeryHigh);
        let pin_rst = Output::new(pc3, Level::Low, Speed::VeryHigh);
        let pin_sck = Output::new(pb1, Level::Low, Speed::VeryHigh);
        let pin_sda = Output::new(pa4, Level::Low, Speed::VeryHigh);
        // Configure row-select pins (initially low/off)
        let row_0 = Output::new(pb2, Level::Low, Speed::VeryHigh);
        let row_1 = Output::new(pa15, Level::Low, Speed::VeryHigh);
        let row_2 = Output::new(pa2, Level::Low, Speed::VeryHigh);
        let row_3 = Output::new(pa7, Level::Low, Speed::VeryHigh);
        let row_4 = Output::new(pa6, Level::Low, Speed::VeryHigh);
        let row_5 = Output::new(pa5, Level::Low, Speed::VeryHigh);
        let row_6 = Output::new(pb0, Level::Low, Speed::VeryHigh);
        let row_7 = Output::new(pa3, Level::Low, Speed::VeryHigh);
        let group_rows: [Output<'_>; 8] = [row_0, row_1, row_2, row_3, row_4, row_5, row_6, row_7];

        // Build the Matrix struct
        let mut my_matrix = Matrix {
            sb: pin_sb,
            lat: pin_lat,
            rst: pin_rst,
            sck: pin_sck,
            sda: pin_sda,
            rows: group_rows,
        };

        // Initial pin states
        my_matrix.rst.set_low(); // Hold reset
        my_matrix.lat.set_high(); // Latch idle high
        my_matrix.sb.set_high(); // Sample/blank idle high
        my_matrix.sck.set_low(); // Clock idle low
        my_matrix.sda.set_low(); // Data idle low
        for row in 0..8 {
            my_matrix.rows[row].set_low(); // All rows off
        }

        // Wait for hardware reset
        Timer::after_millis(100).await;
        my_matrix.rst.set_high(); // Release reset

        // Send the default bank0 waveform
        my_matrix.init_bank0();
        my_matrix.sb.set_high(); // Restore SB
        my_matrix
    }

    /// Generate a short high‑low pulse on SCK to clock one bit.
    fn pulse_sck(&mut self) {
        self.sck.set_high();
        self.sck.set_low();
    }

    /// Generate a short low‑high pulse on LAT to latch data.
    fn pulse_lat(&mut self) {
        self.lat.set_low();
        self.lat.set_high();
    }

    /// Send one byte (8 bits) on SDA, MSB first, pulsing SCK after each bit.
    fn send_byte(&mut self, pixel: u8) {
        for i in 0..8 {
            // Drive SDA depending on the current bit
            self.sda(pixel & (1 << (7 - i)));
            self.pulse_sck();
        }
    }

    /// Send and latch all 8 pixels in a row:
    /// - Deactivate the previous row
    /// - Transmit BGR for each pixel (gamma‑corrected), MSB first
    /// - Activate the current row and pulse LAT
    pub fn send_row(&mut self, row: usize, pixels: &[Color]) {
        if row > 7 {
            panic!("enter a valid row 0–7");
        }
        // Deactivate the row above (wrap from 0 to 7)
        match row {
            0 => self.deactivate_row(7),
            _ => self.deactivate_row(row - 1),
        }
        // Send each pixel in reverse order (hardware expects BGR)
        for pixel in pixels.iter().rev().take(8) {
            let correct = pixel.gamma_correct();
            self.send_byte(correct.b);
            self.send_byte(correct.g);
            self.send_byte(correct.r);
        }
        self.activate_row(row); // Turn on current row
        self.pulse_lat(); // Latch the row data
    }

    /// Initialize bank 0 by clocking 144 “1” bits with SB low, then latch.
    fn init_bank0(&mut self) {
        self.sb.set_low(); // Enter sample mode
        self.sda.set_high(); // Hold data high
        for _ in 0..144 {
            self.pulse_sck(); // Clock a “1” bit
        }
        self.pulse_lat(); // Latch the bank
    }

    /// Display a full 8×8 image as fast as possible, synchronized by the ticker.
    pub async fn display_image(&mut self, image: &Image, ticker: &mut Ticker) {
        // Loop through each row, waiting on ticker to maintain refresh rate
        for i in 0..8 {
            ticker.next().await;
            self.send_row(i, image.row(i));
        }
    }

    /// Turn all rows off.
    pub fn deactivate_rows(&mut self) {
        for row in self.rows.iter_mut() {
            row.set_low();
        }
    }

    /// Turn a single row on.
    pub fn activate_row(&mut self, i: usize) {
        self.rows[i].set_high();
    }

    /// Turn a single row off.
    pub fn deactivate_row(&mut self, i: usize) {
        self.rows[i].set_low();
    }

    /// Drive SDA line: set high if input > 0, else low.
    pub fn sda(&mut self, i: u8) {
        if i > 0 {
            self.sda.set_high();
        } else {
            self.sda.set_low();
        }
    }
}
