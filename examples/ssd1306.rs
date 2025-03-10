#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use panic_probe as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32wb_hal::i2c::I2c;
use stm32wb_hal::{self as hal, prelude::*};

#[entry]
fn main() -> ! {
    let dp = hal::stm32::Peripherals::take().unwrap();

    // Use default clock frequency of 4 MHz running from MSI
    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc);

    let scl = gpioa
        .pa9
        .into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let sda = gpioa
        .pa10
        .into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), &mut rcc);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);

    let im = Image::new(&raw, Point::new(32, 0));

    im.draw(&mut display).unwrap();

    display.flush().unwrap();

    loop {}
}
