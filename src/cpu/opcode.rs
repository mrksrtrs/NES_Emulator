use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct OpCode {
    pub instruction: Instruction,
    pub addr_mode: AddressingMode,
    pub clock_cycles: u8
}

// Addressing modes
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    ADC, ADD, AND, ASL,
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

lazy_static! {

    pub static ref INSTRUCTION_OP_CODE_MATRIX: HashMap<u8, OpCode> = {
        let mut map = HashMap::new();
        
        let nop: OpCode = OpCode { instruction: Instruction::NOP, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 };
    
        // Row: 0x00 - 0x0f
        map.insert(0x00, OpCode { instruction: Instruction::BRK, addr_mode: AddressingMode::Implied, clock_cycles: 0x07 });
        map.insert(0x01, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::XIndirect, clock_cycles: 0x06 });
        map.insert(0x05, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x03 });
        map.insert(0x06, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x05 });
        map.insert(0x08, OpCode { instruction: Instruction::PHP, addr_mode: AddressingMode::Implied, clock_cycles: 0x03 });
        map.insert(0x09, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::Immidiate, clock_cycles: 0x02 });
        map.insert(0x0a, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::Accumulator, clock_cycles: 0x02 });
        map.insert(0x0d, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::Absolute, clock_cycles: 0x04 });
        map.insert(0x0e, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::Absolute, clock_cycles: 0x06 });
    
        // Row: 0x10 - 0x1f
        map.insert(0x10, OpCode { instruction: Instruction::BPL, addr_mode: AddressingMode::Relative, clock_cycles: 0x02 });
        map.insert(0x11, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::IndirectY, clock_cycles: 0x05 });
        map.insert(0x15, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x04 });
        map.insert(0x16, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x06 });
        map.insert(0x18, OpCode { instruction: Instruction::CLC, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 });
        map.insert(0x19, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::AbsoluteY, clock_cycles: 0x04 });
        map.insert(0x1d, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x04 });
        map.insert(0x1e, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x07 });
    
        // Row: 0x20 - 0x2f
        map.insert(0x20, OpCode { instruction: Instruction::JSR, addr_mode: AddressingMode::Absolute, clock_cycles: 0x06 });
        map.insert(0x21, OpCode { instruction: Instruction::AND, addr_mode: AddressingMode::XIndirect, clock_cycles: 0x06 });
        map.insert(0x24, OpCode { instruction: Instruction::BIT, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x03 });
        map.insert(0x25, OpCode { instruction: Instruction::AND, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x03 });
        map.insert(0x26, OpCode { instruction: Instruction::ROL, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x05 });
        map.insert(0x28, OpCode { instruction: Instruction::PLP, addr_mode: AddressingMode::Implied, clock_cycles: 0x04 });
        map.insert(0x29, OpCode { instruction: Instruction::AND, addr_mode: AddressingMode::Immidiate, clock_cycles: 0x02 });
        map.insert(0x2a, OpCode { instruction: Instruction::ROL, addr_mode: AddressingMode::Accumulator, clock_cycles: 0x02 });
        map.insert(0x2c, OpCode { instruction: Instruction::BIT, addr_mode: AddressingMode::Absolute, clock_cycles: 0x04 });
        map.insert(0x2d, OpCode { instruction: Instruction::AND, addr_mode: AddressingMode::Absolute, clock_cycles: 0x04 });
        map.insert(0x2e, OpCode { instruction: Instruction::ROL, addr_mode: AddressingMode::Absolute, clock_cycles: 0x06 });

        // Row: 0x30 - 0x3f
        map.insert(0x30, OpCode { instruction: Instruction::BMI, addr_mode: AddressingMode::Relative, clock_cycles: 0x02 });
        map.insert(0x31, OpCode { instruction: Instruction::AND, addr_mode: AddressingMode::IndirectY, clock_cycles: 0x05 });
        map.insert(0x35, OpCode { instruction: Instruction::AND, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x04 });
        map.insert(0x36, OpCode { instruction: Instruction::ROL, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x06 });
        map.insert(0x38, OpCode { instruction: Instruction::SEC, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 });
        map.insert(0x39, OpCode { instruction: Instruction::AND, addr_mode: AddressingMode::AbsoluteY, clock_cycles: 0x04 });
        map.insert(0x3d, OpCode { instruction: Instruction::AND, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x04 });
        map.insert(0x3e, OpCode { instruction: Instruction::ROL, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x07 });

        // Row: 0x40 - 0x4f
        map.insert(0x40, OpCode { instruction: Instruction::RTI, addr_mode: AddressingMode::Implied, clock_cycles: 0x06 });
        map.insert(0x41, OpCode { instruction: Instruction::EOR, addr_mode: AddressingMode::XIndirect, clock_cycles: 0x06 });
        map.insert(0x45, OpCode { instruction: Instruction::EOR, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x03 });
        map.insert(0x46, OpCode { instruction: Instruction::LSR, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x05 });
        map.insert(0x48, OpCode { instruction: Instruction::PHA, addr_mode: AddressingMode::Implied, clock_cycles: 0x03 });
        map.insert(0x49, OpCode { instruction: Instruction::EOR, addr_mode: AddressingMode::Immidiate, clock_cycles: 0x02 });
        map.insert(0x4a, OpCode { instruction: Instruction::LSR, addr_mode: AddressingMode::Accumulator, clock_cycles: 0x02 });
        map.insert(0x4c, OpCode { instruction: Instruction::JMP, addr_mode: AddressingMode::Absolute, clock_cycles: 0x03 });
        map.insert(0x4d, OpCode { instruction: Instruction::EOR, addr_mode: AddressingMode::Absolute, clock_cycles: 0x04 });
        map.insert(0x4e, OpCode { instruction: Instruction::LSR, addr_mode: AddressingMode::Absolute, clock_cycles: 0x06 });

        // Row: 0x50 - 0x5f
        map.insert(0x50, OpCode { instruction: Instruction::BVC, addr_mode: AddressingMode::Relative, clock_cycles: 0x02 });
        map.insert(0x51, OpCode { instruction: Instruction::EOR, addr_mode: AddressingMode::IndirectY, clock_cycles: 0x05 });
        map.insert(0x55, OpCode { instruction: Instruction::EOR, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x04 });
        map.insert(0x56, OpCode { instruction: Instruction::LSR, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x06 });
        map.insert(0x58, OpCode { instruction: Instruction::CLI, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 });
        map.insert(0x59, OpCode { instruction: Instruction::EOR, addr_mode: AddressingMode::AbsoluteY, clock_cycles: 0x04 });
        map.insert(0x5d, OpCode { instruction: Instruction::EOR, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x04 });
        map.insert(0x5e, OpCode { instruction: Instruction::LSR, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x07 });

        // Row: 0x60 - 0x6f
        map.insert(0x60, OpCode { instruction: Instruction::RTS, addr_mode: AddressingMode::Implied, clock_cycles: 0x06 });
        map.insert(0x61, OpCode { instruction: Instruction::ADC, addr_mode: AddressingMode::XIndirect, clock_cycles: 0x06 });
        map.insert(0x65, OpCode { instruction: Instruction::ADC, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x03 });
        map.insert(0x66, OpCode { instruction: Instruction::ROR, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x05 });
        map.insert(0x68, OpCode { instruction: Instruction::PLA, addr_mode: AddressingMode::Implied, clock_cycles: 0x04 });
        map.insert(0x69, OpCode { instruction: Instruction::ADC, addr_mode: AddressingMode::Immidiate, clock_cycles: 0x02 });
        map.insert(0x6a, OpCode { instruction: Instruction::ROR, addr_mode: AddressingMode::Accumulator, clock_cycles: 0x02 });
        map.insert(0x6c, OpCode { instruction: Instruction::JMP, addr_mode: AddressingMode::Indirect, clock_cycles: 0x05 });
        map.insert(0x6d, OpCode { instruction: Instruction::ADC, addr_mode: AddressingMode::Absolute, clock_cycles: 0x04 });
        map.insert(0x6e, OpCode { instruction: Instruction::ROR, addr_mode: AddressingMode::Absolute, clock_cycles: 0x06 });

        // Row: 0x70 - 0x7f
        map.insert(0x70, OpCode { instruction: Instruction::BVS, addr_mode: AddressingMode::Relative, clock_cycles: 0x02 });
        map.insert(0x71, OpCode { instruction: Instruction::ADC, addr_mode: AddressingMode::IndirectY, clock_cycles: 0x05 });
        map.insert(0x75, OpCode { instruction: Instruction::ADC, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x04 });
        map.insert(0x76, OpCode { instruction: Instruction::ROR, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x06 });
        map.insert(0x78, OpCode { instruction: Instruction::SEI, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 });
        map.insert(0x79, OpCode { instruction: Instruction::ADC, addr_mode: AddressingMode::AbsoluteY, clock_cycles: 0x04 });
        map.insert(0x7d, OpCode { instruction: Instruction::ADC, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x04 });
        map.insert(0x7e, OpCode { instruction: Instruction::ROR, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x07 });

        // Row: 0x80 - 0x8f
        map.insert(0x81, OpCode { instruction: Instruction::STA, addr_mode: AddressingMode::XIndirect, clock_cycles: 0x06 });
        map.insert(0x84, OpCode { instruction: Instruction::STY, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x03 });
        map.insert(0x85, OpCode { instruction: Instruction::STA, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x03 });
        map.insert(0x86, OpCode { instruction: Instruction::STX, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x03 });
        map.insert(0x88, OpCode { instruction: Instruction::DEY, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 });
        map.insert(0x8a, OpCode { instruction: Instruction::TXA, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 });
        map.insert(0x8c, OpCode { instruction: Instruction::STY, addr_mode: AddressingMode::Absolute, clock_cycles: 0x04 });
        map.insert(0x8d, OpCode { instruction: Instruction::STA, addr_mode: AddressingMode::Absolute, clock_cycles: 0x04 });
        map.insert(0x8e, OpCode { instruction: Instruction::STX, addr_mode: AddressingMode::Absolute, clock_cycles: 0x04 });

        // Row: 0x90 - 0x9f
        map.insert(0x90, OpCode { instruction: Instruction::BCC, addr_mode: AddressingMode::Relative, clock_cycles: 0x02 });
        map.insert(0x91, OpCode { instruction: Instruction::STA, addr_mode: AddressingMode::IndirectY, clock_cycles: 0x06 });
        map.insert(0x94, OpCode { instruction: Instruction::STY, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x04 });
        map.insert(0x95, OpCode { instruction: Instruction::STA, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x04 });
        map.insert(0x96, OpCode { instruction: Instruction::STX, addr_mode: AddressingMode::ZeroPageY, clock_cycles: 0x04 });
        map.insert(0x98, OpCode { instruction: Instruction::TYA, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 });
        map.insert(0x99, OpCode { instruction: Instruction::STA, addr_mode: AddressingMode::AbsoluteY, clock_cycles: 0x05 });
        map.insert(0x9a, OpCode { instruction: Instruction::TXS, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 });
        map.insert(0x9d, OpCode { instruction: Instruction::STA, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x05 });

        // Fill map with NOP where its undefined in range 0x00 - 0xff
        for i in 0x00..0xff{
            if ! map.contains_key(&i) {
                map.insert(i, nop);
            }
        }
    
        map
    };
}