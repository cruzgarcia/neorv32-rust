use neorv32_pac::GPIO;

pub struct Gpio {
    registers: GPIO,
}

#[allow(dead_code)]
impl Gpio {
    pub fn new(registers: GPIO) -> Self {
        Self { registers }
    }

    /*
    pub fn set_single(&mut self, red: bool, green: bool,
        hred1: bool,
        hgreen2: bool, hgreen3: bool, hgreen4: bool, hgreen5: bool) {
        self.registers.out.write(|w| {
            w.ledr().bit(red);
            w.ledg().bit(green);
            w.hledr1().bit(hred1);
            w.hledg2().bit(hgreen2);
            w.hledg3().bit(hgreen3);
            w.hledg4().bit(hgreen4);
            w.hledg5().bit(hgreen5)
        });
    }
*/ 
    pub fn set(&mut self, gpio: u32) {
        unsafe {
            self.registers.output_lo.write(|w| w.bits(gpio));
        }
    }

    pub fn off(&mut self) {
        unsafe {
            self.registers.output_lo.write(|w| w.bits(0x0000_0000));
        }
    }

    pub fn on(&mut self) {
        unsafe {
            self.registers.output_lo.write(|w| w.bits(0xFFFF_FFFF));
        }
    }

    pub fn toggle(&mut self) {
        self.toggle_mask(0xFFFF_FFFF);
    }

    pub fn toggle_mask(&mut self, mask: u32) {
        let val: u32 = self.registers.output_lo.read().bits() ^ mask;
        unsafe {
            self.registers.output_lo.write(|w| w.bits(val));
        }
    }
}