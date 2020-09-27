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

lazy_static! {

    pub static ref INSTRUCTION_OP_CODE_MATRIX: HashMap<u8, OpCode> = {
        let mut map = HashMap::new();
        
        let nop: OpCode = OpCode { instruction: Instruction::NOP, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 };
    
        // First row: 0x00 - 0x0f
        map.insert(0x00, OpCode { instruction: Instruction::BRK, addr_mode: AddressingMode::Implied, clock_cycles: 0x07 });
        map.insert(0x01, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::XIndirect, clock_cycles: 0x06 });
        map.insert(0x05, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x03 });
        map.insert(0x06, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::ZeroPage, clock_cycles: 0x05 });
        map.insert(0x08, OpCode { instruction: Instruction::PHP, addr_mode: AddressingMode::Implied, clock_cycles: 0x03 });
        map.insert(0x09, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::Immidiate, clock_cycles: 0x02 });
        map.insert(0x0a, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::Accumulator, clock_cycles: 0x02 });
        map.insert(0x0d, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::Absolute, clock_cycles: 0x04 });
        map.insert(0x0e, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::Absolute, clock_cycles: 0x06 });
    
        // Second row: 0x10 - 0x1f
        map.insert(0x10, OpCode { instruction: Instruction::BPL, addr_mode: AddressingMode::Relative, clock_cycles: 0x02 });
        map.insert(0x11, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::IndirectY, clock_cycles: 0x05 });
        map.insert(0x15, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x04 });
        map.insert(0x16, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::ZeroPageX, clock_cycles: 0x06 });
        map.insert(0x18, OpCode { instruction: Instruction::CLC, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 });
        map.insert(0x19, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::AbsoluteY, clock_cycles: 0x04 });
        map.insert(0x1d, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x04 });
        map.insert(0x1e, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode::AbsoluteX, clock_cycles: 0x07 });
    
        // Third row: 0x20 - 0x2f
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

        // Fill map with NOP where its undefined in range 0x00 - 0xff
        for i in 0x00..0xff{
            if ! map.contains_key(&i) {
                map.insert(i, nop);
            }
        }
    
        map
    };
}