mod bus;
mod cpu;
mod memory;

fn main() {
    println!("Hello, world!");
    let program: Vec<u8> = vec![0x06, 0x3F, 0x80, 0x16, 0x1F, 0x92];
    let memory = crate::memory::MEMORY::new();
    let bus = crate::bus::BUS::new(memory);
    let mut cpu = crate::cpu::CPU::new(bus);
    //cpu.flip_flag(crate::cpu::flags::Z_FLAG);
    cpu.bus.load(program);
    // cpu.build_opcode_table();
    cpu.run(true)
}
