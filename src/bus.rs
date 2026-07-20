pub struct Bus {
    pub wram: [u8; 8192],
}

impl Bus {
    pub fn new() -> Self {
        Bus { wram: [0; 8192] }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            _ => {}
        }
    }
}
