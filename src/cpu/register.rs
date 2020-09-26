pub struct Register {
    // Accumulator register
    pub acc: u8,
    // Index register X
    pub x: u8,
    // Index register Y
    pub y: u8,
    // Stack pointer register
    pub sp: u8,
    // Program counter
    pub pcl: u16,
    // Processor status register
    pub status: u8
}