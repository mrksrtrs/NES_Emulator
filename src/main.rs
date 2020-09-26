mod cpu6502;

fn main() {
    
    let cpu = cpu6502::new();

    println!("Content of x_reg {}", cpu.reg_x);

}
