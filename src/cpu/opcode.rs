pub struct OpCode {
    pub instruction: Instruction,
    pub addr_mode: AddressingMode,
    pub clock_cycles: u8
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

pub static ref INSTRUCTION_OP_CODE_MATRIX: Map<u8, OpCode> = {
    let mut map = HashMap::new();
    
    let nop: OpCode = OpCode { instruction: Instruction::NOP, addr_mode: AddressingMode::Implied, clock_cycles: 0x02 };

    // First row: 0x00 - 0x0f
    map.insert(0x00, OpCode { instruction: Instruction::BRK, addr_mode: AddressingMode::Implied, clock_cycles: 0x07 });
    map.insert(0x01, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode:XIndirect, clock_cycles: 0x06 });
    map.insert(0x05, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode:ZeroPage, clock_cycles: 0x03 });
    map.insert(0x06, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode:ZeroPage, clock_cycles: 0x05 });
    map.insert(0x08, OpCode { instruction: Instruction::PHP, addr_mode: AddressingMode:Implied, clock_cycles: 0x03 });
    map.insert(0x09, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode:Immidiate, clock_cycles: 0x02 });
    map.insert(0x0a, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode:Accumulator, clock_cycles: 0x02 });
    map.insert(0x0d, OpCode { instruction: Instruction::ORA, addr_mode: AddressingMode:Absolute, clock_cycles: 0x04 });
    map.insert(0x0e, OpCode { instruction: Instruction::ASL, addr_mode: AddressingMode:Absolute, clock_cycles: 0x06 });

    // Fill map with NOP where its undefined in range 0x00 - 0xff
    for i in 0x00..0xff{
        if ! map.contains_key(i) {
            map.insert(i, nop);
        }
    }

    map
}