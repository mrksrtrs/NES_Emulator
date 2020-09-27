pub mod cpu;
pub mod bus;

fn main() {
    
    let bus = bus::Bus::new();
    let cpu = cpu::cpu6502::Cpu6502::new(bus);

    let op_codes = &cpu::opcode::INSTRUCTION_OP_CODE_MATRIX;
    println!("Size: {}, correct size {}", op_codes.len(), op_codes.len() == (16 * 16));
    for (k, opc) in op_codes.iter() {
        println!("{}: AddrMode: {:?}, Instruction: {:?}, Num of cycles: {}", k, opc.addr_mode, opc.instruction, opc.clock_cycles);
    }

}
