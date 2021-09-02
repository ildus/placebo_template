#![no_std]
#![no_main]
#![deny(warnings)]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate rtic;
extern crate stm32g0xx_hal as hal;

 use defmt_rtt as _;

use hal::gpio::{gpioc::*, *};
use hal::prelude::*;
use hal::rcc;
use hal::stm32;
use hal::timer::*;

#[rtic::app(device = hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        led: PC15<Output<OpenDrain>>,
        timer: Timer<stm32::TIM2>,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let mut rcc = ctx.device.RCC.freeze(rcc::Config::pll());
        defmt::info!("init");

        let port_c = ctx.device.GPIOC.split(&mut rcc);
        let led = port_c.pc15.into_open_drain_output();

        let mut timer = ctx.device.TIM2.timer(&mut rcc);
        timer.start(2.hz());
        timer.listen();

        defmt::info!("start");
        init::LateResources { timer, led }
    }

    #[task(binds = TIM2, resources = [timer, led])]
    fn timer_tick(ctx: timer_tick::Context) {
        let timer_tick::Resources { led, timer } = ctx.resources;

        defmt::info!("timer tick");
        led.toggle().ok();
        timer.clear_irq();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
};
