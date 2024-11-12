#![no_std]
#![no_main]

use cortex_m::asm;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
// use cortex_m_semihosting::hprint;
use stm32f0::stm32f0x0::{interrupt, Interrupt, NVIC};
use stm32f0xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.configure()
        .sysclk(8.mhz())
        .freeze(&mut p.FLASH);

    let gpioa = p.GPIOA.split(&mut rcc);

    let mut timer = stm32f0xx_hal::timers::Timer::tim1(p.TIM1, 60.hz(), &mut rcc);
    
    unsafe { NVIC::unmask(Interrupt::EXTI0_1); }

    // configure the system timer to wrap around every second
    // syst.set_clock_source(SystClkSource::Core);
    // syst.set_reload(8_000_000); // 1s
    // syst.enable_counter();

    loop {
        // // busy wait until the timer wraps around
        // while !syst.has_wrapped() {}

        // trigger the `EXTI0` interrupt
        NVIC::pend(Interrupt::EXTI0_1);
    }
}

#[interrupt]
fn EXTI0_1() {
    asm::nop();
    // hprint!(".").unwrap();
}

