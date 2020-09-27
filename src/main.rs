pub mod cpu;
pub mod bus;

fn main() {
    
    let bus = bus::Bus::new();
    let cpu = cpu::cpu6502::new(bus);

    let op_codes = &cpu::opcode::INSTRUCTION_OP_CODE_MATRIX;
    println!("Size: {}", op_codes.len());
    for (k, opc) in op_codes.iter() {
        println!("{}: AddrMode: {:?}, Instruction: {:?}, Num of cycles: {}", k, opc.addr_mode, opc.instruction, opc.clock_cycles);
    }

}
