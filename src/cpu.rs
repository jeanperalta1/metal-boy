use crate::bus::Bus;

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

    // Reading a 16-bit pair
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    // Writing a 16-bit pair
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    // Reading a 16-bit pair
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    // Writing a 16-bit pair
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xF0) as u8;
    }

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

    pub fn set_flag_z(&mut self, on: bool) {
        if on {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }
    }

    pub fn set_flag_n(&mut self, on: bool) {
        if on {
            self.f |= 0x40;
        } else {
            self.f &= !0x40;
        }
    }

    pub fn set_flag_h(&mut self, on: bool) {
        if on {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
    }

    pub fn set_flag_c(&mut self, on: bool) {
        if on {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }
    }
}

pub struct CPU {
    pub registers: Registers,
    pub cycles: u32,
    pub halted: bool,
    pub bus: Bus,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers {
                a: 0x01,
                f: 0xB0,
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                sp: 0xFFFE,
                pc: 0x0100,
            },
            cycles: 0,
            halted: false,
            bus: Bus::new(),
        }
    }

    // Represents a full CPU cycle
    pub fn step(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    /*
     * 1. Reads byte sitting at whatever address PC is pointing too and that byte is the next
     *    instruction
     * 2. Increments PC to point to the next byte
     */
    fn fetch(&mut self) -> u8 {
        let byte = self.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    /*
     * Takes the opcode byte that fetch returned and decides what to do with it
     * Matches 0x00 -> NOP -> adds 4 cycles
     */
    fn execute(&mut self, opcode: u8) {
        match opcode {
            0x00 => {
                self.cycles += 4;
            }
            // Implementing the Load Instructions
            // LD A, n: Fetch next byte and put into A
            0x06 => {
                self.registers.b = self.fetch();
                self.cycles += 8;
            }
            0x0E => {
                self.registers.c = self.fetch();
                self.cycles += 8;
            }
            0x16 => {
                self.registers.d = self.fetch();
                self.cycles += 8;
            }
            0x1E => {
                self.registers.e = self.fetch();
                self.cycles += 8;
            }
            0x26 => {
                self.registers.h = self.fetch();
                self.cycles += 8;
            }
            0x2E => {
                self.registers.l = self.fetch();
                self.cycles += 8;
            }
            0x3E => {
                self.registers.a = self.fetch();
                self.cycles += 8;
            }

            _ => {
                panic!("Unknown opcode: 0x{:02X}", opcode);
            }
        }
    }
}
