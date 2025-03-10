//! Reads data from a BME280 over i2c
//!
//! This assumes that a BME280 is connected with clk on PB6 and data on PB7.
//!
//! For the Adafruit breakout boards PB6 should be connected to SCK and PB7 to SDI
//!
//! This program writes the sensor values to the debug output provided by semihosting
//! you must enable semihosting in gdb with `monitor arm semihosting enable` I have it
//! added to my `.gdbinit`. Then the debug infomation will be printed in your openocd
//! terminal.
//!
//! This program dose not fit on my blue pill unless compiled in release mode
//! eg. `cargo run --example i2c-bme280 --features "stm32f103 bme280 rt" --release`
//! However as noted above the debug output with the read values will be in the openocd
//! terminal.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use bme280::i2c::BME280;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_probe as _;
use stm32wb_hal::i2c::I2c;
use stm32wb_hal::prelude::_stm32wb_hal_time_U32Ext;
use stm32wb_hal::{self as hal, delay, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    // Use default clock frequency of 4 MHz running from MSI
    let mut rcc = dp.RCC.constrain();

    let delay = delay::Delay::new(cp.SYST, rcc.clocks.clone());

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

    // The Adafruit boards have address 0x77 without closing the jumper on the back, the BME280 lib connects to 0x77 with `new_secondary`, use
    // `new_primary` for 0x76 if you close the jumper/solder bridge.
    let mut bme280 = BME280::new_secondary(i2c, delay);
    bme280
        .init()
        .map_err(|error| {
            hprintln!("Could not initialize bme280, Error: {:?}", error);
            panic!();
        })
        .unwrap();
    loop {
        match bme280.measure() {
            Ok(measurements) => {
                hprintln!("Relative Humidity = {}%", measurements.humidity);
                hprintln!("Temperature = {} deg C", measurements.temperature);
                hprintln!("Pressure = {} pascals", measurements.pressure)
            }
            Err(error) => {
                hprintln!("Could not read bme280 due to error: {:?}", error);
            }
        }
    }
}
