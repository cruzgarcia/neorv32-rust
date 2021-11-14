#![no_std]
#![no_main]

extern crate panic_halt;

use neorv32_pac;
use riscv_rt::entry;

//mod timer;
mod leds;
mod print;

//use timer::Timer;
use leds::Leds;

const SYSTEM_CLOCK_FREQUENCY: u32 = 48_000_000;

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = neorv32_pac::Peripherals::take().unwrap();

    //let mut timer = Timer::new(peripherals.TIMER0);
    let mut leds = Leds::new(peripherals.LEDS);
    leds.set_single(false, false, false, false, false, false, false);
    //print::print_hardware::set_hardware(peripherals.UART);
    
    loop {
        //print!("a");
        leds.toggle();
        //msleep(&mut timer, 160);
    }
    
}

//fn msleep(timer: &mut Timer, ms: u32) {
//    timer.disable();
//
//    timer.reload(0);
//    timer.load(SYSTEM_CLOCK_FREQUENCY / 1_000 * ms);
//
//    timer.enable();
//
//    // Wait until the time has elapsed
//    while timer.value() > 0 {}
//}
