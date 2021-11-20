use neorv32_pac::UART;

pub struct Uart {
    pub registers: Option<UART>,
}

impl Uart {

    pub fn putc(&self, c: u8) {

        match self.registers.as_ref() {
            Some(reg) => unsafe {
                // Chec FIFO full
                while reg.ctrl.read().tx_full().bit() == true {
                    ()
                }
                // Write data
                reg.rxtx_data.write(|w| w.rxtx_data().bits(c) );
            },
            None => ()
        }
    }
}

use core::fmt::{Error, Write};
impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for c in s.bytes() {
            self.putc(c);
        }
        Ok(())
    }
}

#[macro_use]
#[cfg(not(test))]
pub mod print_hardware {

    use crate::print::*;

    pub static mut SUPERVISOR_UART: Uart = Uart {
        registers: None,
    };

    pub fn set_hardware(uart: UART) {
        let baud : u16  = 0x4E1;
        unsafe {
            SUPERVISOR_UART.registers = Some(uart);
            // Config
            //SUPERVISOR_UART.registers.as_ref().unwrap().ctrl.write(|w| w.baud().bits(baud));
            //SUPERVISOR_UART.registers.as_ref().unwrap().ctrl.write(|w| w.rts_en().bit(false));
            //SUPERVISOR_UART.registers.as_ref().unwrap().ctrl.write(|w| w.cts_en().bit(false));
            //SUPERVISOR_UART.registers.as_ref().unwrap().ctrl.write(|w| w.en().bit(true));
        }
    }

    #[macro_export]
    macro_rules! print
    {
        ($($args:tt)+) => ({
                use core::fmt::Write;
                unsafe {
                    let _ = write!(crate::print::print_hardware::SUPERVISOR_UART, $($args)+);
                }
        });
    }
}

#[macro_export]
macro_rules! println
{
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}

