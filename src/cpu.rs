/*
 * Create and model the registers
 * 8-bit registers: A B C D E F H L
 * 16-bit registers: SP (Stack Pointer) PC (Program Counter)
 * Flags register: F where 7 = Z, 6 = N, 5 = H, 4 = C
 */
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

/*
 * Implement paired registers and flag helpers
 */
impl Registers {
    // Reading a 16-bit pair
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    // Writing a 16-bit pair
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    // Reading a 16-bit pair
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    // Writing a 16-bit pair
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn hl(&mut self, value: u16) {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl() {}

    pub fn af() {}

    pub fn set_af() {}

    /*
     * Flag helpers
     */
    pub fn flag_z(&self) -> bool {
        self.f & 0x80 != 0
    }

    pub fn flag_n(&self) -> bool {
        self.f & 0x40 != 0
    }

    pub fn flag_h(&self) -> bool {
        self.f & 0x20 != 0
    }

    pub fn flag_c(&self) -> bool {
        self.f & 0x10 != 0
    }
}
