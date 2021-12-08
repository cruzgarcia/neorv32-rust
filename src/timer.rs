use neorv32_pac::MTIME;

pub struct Timer {
    registers: MTIME,
}

impl Timer {
    pub fn new(registers: MTIME) -> Self {
        Self { registers }
    }

/*
    pub fn enable(&mut self) {
        unsafe {
            self.registers.en.write(|w| w.bits(1));
        }
    }

    pub fn disable(&mut self) {
        unsafe {
            self.registers.en.write(|w| w.bits(0));
        }
    }
*/
    pub fn load(&mut self, value: u32) {
        unsafe {
            self.registers.time_lo.write(|w| w.bits(value));
            self.registers.time_hi.write(|w| w.bits(0));
        }
    }

    /*
    pub fn reload(&mut self, value: u32) {
        unsafe {
            self.registers.reload.write(|w| w.bits(value));
        }
    }
    */

    pub fn value(&mut self) -> u32 {
        /*
        unsafe {
            self.registers.update_value.write(|w| w.bits(1));
        }
        */
        //self.registers.value.read().bits()
        self.registers.time_lo.read().bits()

    }
}
