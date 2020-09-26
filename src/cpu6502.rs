pub struct Cpu6502 {
    // Accumulator register
    pub acc: u8,
    // Index register X
    pub reg_x: u8,
    // Index register Y
    pub reg_y: u8,
    // Stack pointer register
    pub sp: u8,
    // Program counter
    pub pcl: u16,
    // Processor status register
    pub status: u8
}

pub fn new() -> Cpu6502 {
    Cpu6502 {
        acc: 0x00,
        reg_x: 0x00,
        reg_y: 0x00,
        sp: 0x00,
        pcl: 0x0000,
        status: 0x00
    }
}