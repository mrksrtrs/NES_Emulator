const RAM_SIZE: usize = 64 * 1024;

pub struct Bus {
    // Fake ram (TODO: change for real ram later)
    pub ram: [u8; RAM_SIZE]
}

impl Bus {

    pub fn new() -> Self {
        Self {
            ram: [0x00; RAM_SIZE]
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        self.ram[addr as usize] = data;
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }
    
}
