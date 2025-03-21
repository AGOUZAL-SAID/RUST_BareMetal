use crate::{Color, Image};
use embassy_stm32::{
    gpio::*,
    peripherals::{PA2, PA3, PA4, PA5, PA6, PA7, PA15, PB0, PB1, PB2, PC3, PC4, PC5},
};
use embassy_time::Delay;
use embedded_hal::delay::DelayNs as _;

pub struct Matrix<'a> {
    sb: Output<'a>,
    lat: Output<'a>,
    rst: Output<'a>,
    sck: Output<'a>,
    sda: Output<'a>,
    rows: [Output<'a>; 8],
}

impl Matrix<'_> {
    /// Create a new matrix from the control registers and the individual
    /// unconfigured pins. SB and LAT will be set high by default, while
    /// other pins will be set low. After 100ms, RST will be set high, and
    /// the bank 0 will be initialized by calling `init_bank0()` on the
    /// newly constructed structure.
    /// The pins will be set to very high speed mode.
    #[allow(clippy::too_many_arguments)] // Necessary to avoid a clippy warning
    pub fn new(
        pa2: PA2,
        pa3: PA3,
        pa4: PA4,
        pa5: PA5,
        pa6: PA6,
        pa7: PA7,
        pa15: PA15, // <Alternate<PushPull, 0>>,
        pb0: PB0,
        pb1: PB1,
        pb2: PB2,
        pc3: PC3,
        pc4: PC4,
        pc5: PC5,
    ) -> Self {
        // Configure the pins, with the correct speed and their initial state
        let pin_sb = Output::new(pc5, Level::Low, Speed::VeryHigh);
        let pin_lat = Output::new(pc4, Level::Low, Speed::VeryHigh);
        let pin_rst = Output::new(pc3, Level::Low, Speed::VeryHigh);
        let pin_sck = Output::new(pb1, Level::Low, Speed::VeryHigh);
        let pin_sda = Output::new(pa4, Level::Low, Speed::VeryHigh);
        let row_0 = Output::new(pb2, Level::Low, Speed::VeryHigh);
        let row_1 = Output::new(pa15, Level::Low, Speed::VeryHigh);
        let row_2 = Output::new(pa2, Level::Low, Speed::VeryHigh);
        let row_3 = Output::new(pa7, Level::Low, Speed::VeryHigh);
        let row_4 = Output::new(pa6, Level::Low, Speed::VeryHigh);
        let row_5 = Output::new(pa5, Level::Low, Speed::VeryHigh);
        let row_6 = Output::new(pb0, Level::Low, Speed::VeryHigh);
        let row_7 = Output::new(pa3, Level::Low, Speed::VeryHigh);
        let group_rows: [Output<'_>; 8] = [row_0, row_1, row_2, row_3, row_4, row_5, row_6, row_7];
        let mut my_matrix = Matrix {
            sb: pin_sb,
            lat: pin_lat,
            rst: pin_rst,
            sck: pin_sck,
            sda: pin_sda,
            rows: group_rows,
        };
        my_matrix.rst.set_low();
        my_matrix.lat.set_high();
        my_matrix.sb.set_high();
        my_matrix.sck.set_low();
        my_matrix.sda.set_low();
        for i in 0..8 {
            my_matrix.rows[i].set_low();
        }
        Delay.delay_ms(100);
        my_matrix.rst.set_high();
        my_matrix.init_bank0();
        my_matrix.sb.set_high();
        my_matrix
    }

    /// Make a brief high pulse of the SCK pin
    fn pulse_sck(&mut self) {
        self.sck.set_high();
        self.sck.set_low();
    }

    /// Make a brief low pulse of the LAT pin
    fn pulse_lat(&mut self) {
        self.lat.set_low();
        self.lat.set_high();
    }

    /// Send a byte on SDA starting with the MSB and pulse SCK high after each bit
    fn send_byte(&mut self, pixel: u8) {
        for i in 0..8 {
            self.sda(pixel & (1 << (7 - i)));
            self.pulse_sck();
        }
    }

    /// Send a full row of bytes in BGR order and pulse LAT low. Gamma correction
    /// must be applied to every pixel before sending them. The previous row must
    /// be deactivated and the new one activated.
    pub fn send_row(&mut self, row: usize, pixels: &[Color]) {
        if row > 7 {
            panic!("enter a valide row U monster")
        }
        match row {
            0 => self.deactivate_row(7),
            _ => self.deactivate_row(row - 1),
        }
        for pixel in pixels.iter().take(8) {
            let correct_gamma = pixel.gamma_correct();
            self.send_byte(correct_gamma.b);
            self.send_byte(correct_gamma.g);
            self.send_byte(correct_gamma.r);
        }
        self.activate_row(row);
        self.pulse_lat();
    }

    /// Initialize bank0 by temporarily setting SB to low and sending 144 one bits,
    /// pulsing SCK high after each bit and pulsing LAT low at the end. SB is then
    /// restored to high.
    fn init_bank0(&mut self) {
        self.sb.set_low();
        self.sda.set_high();
        for _ in 0..144 {
            self.pulse_sck();
        }
        self.pulse_lat();
    }

    /// Display a full image, row by row, as fast as possible.
    pub fn display_image(&mut self, image: &Image) {
        // Do not forget that image.row(n) gives access to the content of row n,
        // and that self.send_row() uses the same format.
        loop {
            for i in 0..8 {
                self.send_row(i, image.row(i));
            }
        }
    }
    pub fn deactivate_rows(&mut self) {
        for i in 0..8 {
            self.rows[i].set_low();
        }
    }

    pub fn activate_row(&mut self, i: usize) {
        if i > 7 {
            self.deactivate_rows();
            panic!("enter a valide row U monster")
        }
        self.rows[i].set_high();
    }
    pub fn deactivate_row(&mut self, i: usize) {
        if i > 7 {
            self.deactivate_rows();
            panic!("enter a valide row U monster")
        }
        self.rows[i].set_low();
    }

    pub fn sda(&mut self, i: u8) {
        if i > 0 {
            self.sda.set_high();
        } else {
            self.sda.set_low();
        }
    }
}
