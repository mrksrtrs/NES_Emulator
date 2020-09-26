use crate::cpu::register;
use crate::bus::Bus;

pub struct Cpu6502 {
    // CPU registers
    pub registers: register::Register,
    // Communication bus
    pub bus: Bus
}

pub fn new(bus: Bus) -> Cpu6502 {
    Cpu6502 {
        registers: register::Register {
            acc: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0x00,
            pcl: 0x0000,
            status: 0x00,
        },
        bus: bus
    }
}

pub fn clock_cycle() {}
pub fn reset() {}
pub fn irq() {}
pub fn nmi() {}

pub enum StatusRegisterFlags {
    // Carry, 1 = True
    C = 1 << 0,
    // Zero, 1 = Result Zero
    Z = 1 << 1,
    // IRQ disable, 1 = Disable
    I = 1 << 2,
    // Decimal mode, 1 = True
    D = 1 << 3,
    // BRK Command, 1 = Break
    B = 1 << 4,
    // Unused
    U = 1 << 5,
    // Overflow, 1 = True
    V = 1 << 6,
    // Negative, 1 = Negative
    N = 1 << 7
}
