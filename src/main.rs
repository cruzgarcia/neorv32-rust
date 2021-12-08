#![no_std]
#![no_main]

extern crate panic_halt;

use neorv32_pac;
use riscv_rt::entry;

//mod timer;
mod gpio;
mod print;

//use timer::Timer;
use gpio::Gpio;

const SYSTEM_CLOCK_FREQUENCY: u32 = 48_000_000;

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = neorv32_pac::Peripherals::take().unwrap();
    //let mut timer = Timer::new(peripherals.MTIME);
    let mut gpio = Gpio::new(peripherals.GPIO);
    gpio.set(0);
    print::print_hardware::set_hardware(peripherals.UART);
    let mut cnt = 0;

    // Test code, only debug prints
    loop {
        gpio.toggle();
        print!("Hello world!\n");

        let mut i = 0;
        while i < 1000000 {
            i = i + 1;
        }
        //print!("Count: {}\n", cnt);
        //msleep(&mut timer, 160);
        cnt = cnt + 1;
    }
    
}

//fn msleep(timer: &mut Timer, ms: u32) {
//    timer.disable();
//    timer.reload(0);
//    timer.load(SYSTEM_CLOCK_FREQUENCY / 1_000 * ms);
//    timer.enable();
//    // Wait until the time has elapsed
//    while timer.value() > 0 {}
//}
