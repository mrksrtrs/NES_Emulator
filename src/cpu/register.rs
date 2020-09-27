pub struct Registers {
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

pub trait CpuRegisters {
    fn get_acc(&self) -> u8;
    fn get_x(&self) -> u8;
    fn get_y(&self) -> u8;
    fn get_sp(&self) -> u8;
    fn get_pcl(&self) -> u16;
    fn get_status(&self) -> u8;
    fn set_acc(&mut self, acc: u8) -> &mut Self;
    fn set_x(&mut self, x: u8) -> &mut Self;
    fn set_y(&mut self, y: u8) -> &mut Self;
    fn set_sp(&mut self, sp: u8) -> &mut Self;
    fn set_pcl(&mut self, pcl: u16) -> &mut Self;
    fn set_status(&mut self, status: u8) -> &mut Self;
}

impl CpuRegisters for Registers {
    
    fn get_acc(&self) -> u8 {
        self.acc
    }

    fn get_x(&self) -> u8 {
        self.x
    }

    fn get_y(&self) -> u8 {
        self.y
    }

    fn get_sp(&self) -> u8 {
        self.sp
    }

    fn get_pcl(&self) -> u16 {
        self.pcl
    }

    fn get_status(&self) -> u8 {
        self.status
    }

    fn set_acc(&mut self, acc: u8) -> &mut Self {
        self.acc = acc;
        self
    }

    fn set_x(&mut self, x: u8) -> &mut Self {
        self.x = x;
        self
    }

    fn set_y(&mut self, y: u8) -> &mut Self {
        self.y = y;
        self
    }

    fn set_sp(&mut self, sp: u8) -> &mut Self {
        self.sp = sp;
        self
    }

    fn set_pcl(&mut self, pcl: u16) -> &mut Self {
        self.pcl = pcl;
        self
    }
    
    fn set_status(&mut self, status: u8) -> &mut Self {
        self.status = status;
        self
    }

}