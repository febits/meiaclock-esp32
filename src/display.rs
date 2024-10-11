use anyhow::Result;

use embedded_graphics::{
    image::Image,
    mono_font::{ascii::FONT_10X20, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306_i2c::{prelude::*, Builder};
use tinybmp::Bmp;

use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::i2c::I2C0;
use esp_idf_svc::hal::i2c::*;
use esp_idf_svc::hal::prelude::*;

use crate::api::SubathonTimer;

const SSD1306_ADDR: u8 = 0x3c;

pub struct Display<'a> {
    ddriver: GraphicsMode<I2cInterface<I2cDriver<'a>>>,
    text_style: MonoTextStyle<'a, BinaryColor>,
}

impl Display<'_> {
    pub fn new(i2cp: I2C0, sda: AnyIOPin, scl: AnyIOPin) -> Result<Self> {
        let config = I2cConfig::new().baudrate(400.kHz().into());
        let i2c = I2cDriver::new(i2cp, sda, scl, &config)?;

        Ok(Self {
            ddriver: Builder::new()
                .with_size(DisplaySize::Display128x64NoOffset)
                .with_i2c_addr(SSD1306_ADDR)
                .with_rotation(DisplayRotation::Rotate0)
                .connect_i2c(i2c)
                .into(),
            text_style: MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .text_color(BinaryColor::On)
                .build(),
        })
    }

    pub fn init_display(&mut self) {
        self.ddriver.init().unwrap();
    }

    pub fn draw_meianatal(&mut self) {
        self.ddriver.flush().unwrap();
        self.ddriver.clear();

        let bmp: Bmp<'_, BinaryColor> =
            Bmp::from_slice(include_bytes!("../resources/meianatalthon.bmp")).unwrap();
        let image = Image::new(&bmp, Point::zero());

        image.draw(&mut self.ddriver).unwrap();

        self.ddriver.flush().unwrap();
    }

    pub fn draw_timer(&mut self, timer: &SubathonTimer) {
        self.ddriver.flush().unwrap();
        self.ddriver.clear();

        Text::with_baseline(
            format!(
                "{:02}:{:02}:{:02}",
                timer.hours, timer.minutes, timer.seconds
            )
            .as_str(),
            Point::new(12, 10),
            self.text_style,
            Baseline::Top,
        )
        .draw(&mut self.ddriver)
        .unwrap();

        Text::with_baseline(":>", Point::new(50, 40), self.text_style, Baseline::Top)
            .draw(&mut self.ddriver)
            .unwrap();

        self.ddriver.flush().unwrap();
    }
}
