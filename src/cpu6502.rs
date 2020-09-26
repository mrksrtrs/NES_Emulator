use crate::bus::Bus;

pub struct Cpu6502 {
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
    pub status: u8,
    // Communication bus
    pub bus: Bus
}

pub fn new(bus: Bus) -> Cpu6502 {
    Cpu6502 {
        acc: 0x00,
        x: 0x00,
        y: 0x00,
        sp: 0x00,
        pcl: 0x0000,
        status: 0x00,
        bus: bus
    }
}

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

// Addressing modes
pub enum AddressingMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immidiate,
    Implied,
    Indirect,
    XIndirect,
    IndirectY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY
}

// OP Codes
pub enum Instruction {
    ADD, AND, ASL,
    BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS,
    CLC, CLD, CLI, CLV, CMP, CPX, CPY,
    DEC, DEX, DEY,
    EOR,
    INC, INX, INY,
    JMP, JSR,
    LDA, LDX, LDY, LSR,
    NOP,
    ORA,
    PHA, PHP, PLA, PLP,
    ROL, ROR, RTI, RTS,
    SBC, SEC, SED, SEI, STA, STX, STY,
    TAX, TAY, TSX, TXA, TXS, TYA 
}