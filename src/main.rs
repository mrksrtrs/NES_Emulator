pub mod cpu;
pub mod bus;

fn main() {
    
    let bus = bus::Bus::new();
    let cpu = cpu::cpu6502::new(bus);
    
    let c = 1 << 3;
    println!("ci s {}", c);
    
}
