#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_probe as _;
use stm32wb_hal::{self as hal, delay::Delay, prelude::*};

#[entry]
fn main() -> ! {
    hprintln!("Starting...");
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    // Use default clock frequency of 4 MHz running from MSI
    let mut rcc = dp.RCC.constrain();

    // On WeAct STM32WB55CGU6 blue LED is connected to the pin PE4
    let mut gpioe = dp.GPIOE.split(&mut rcc);
    let mut led = gpioe
        .pe4
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let mut timer = Delay::new(cp.SYST, hal::rcc::Clocks::default());
    loop {
        timer.delay_ms(500 as u32);
        let _ = led.set_high();
        timer.delay_ms(500 as u32);
        let _ = led.set_low();
    }
}
