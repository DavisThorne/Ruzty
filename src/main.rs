mod cpu;
mod memory;
mod bus;


fn main() {
    println!("Hello, world!");
    let program: Vec<u8> = vec![0x01, 0x11, 0x11,
                                0x40,
                                0x50,
                                0x10];
    let memory = crate::memory::MEMORY::new();
    let bus = crate::bus::BUS::new(memory);
    let mut cpu = crate::cpu::CPU::new(bus);
    //cpu.flip_flag(crate::cpu::flags::Z_FLAG);
    cpu.bus.load(program);
    cpu.build_opcode_table();
    cpu.run(true)

}