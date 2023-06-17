#![no_main]
#![no_std]

// use nb::block;
use panic_halt as _;
// use stm32f4xx_hal::timer::Timer;
use stm32f4xx_hal::{gpio::Edge, pac, prelude::*};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(25.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    let gpioa = p.GPIOA.split();
    let mut button = gpioa.pa0.into_input();

    let gpioc = p.GPIOC.split();
    let mut led_pin = gpioc.pc13.into_push_pull_output();
    led_pin.set_high();

    button.make_interrupt_source(&mut p.SYSCFG.constrain());
    button.enable_interrupt(&mut p.EXTI);
    button.trigger_on_edge(&mut p.EXTI, Edge::Rising);

    loop {
        if button.check_interrupt() {
            led_pin.toggle();
            delay.delay_ms(1000_u32);
            button.clear_interrupt_pending_bit();
        }
    }
}
