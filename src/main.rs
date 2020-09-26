pub mod cpu6502;
pub mod bus;

fn main() {
    
    let bus = bus::Bus::new();
    let cpu = cpu6502::new(bus);

    println!("Content of x_reg {}", cpu.x);

    let c = 1 << 3;
    println!("ci s {}", c);
    
}
