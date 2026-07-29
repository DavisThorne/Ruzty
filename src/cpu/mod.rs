#[doc(inline)]
use std::io;
pub mod flags;
mod opcodes;
mod registers;
use crate::bus::BUS;
use crate::cpu::flags::*;

// const Z_FLAG: u16 = 0x0080;
// const N_FLAG: u16 = 0x0040;
// const H_FLAG: u16 = 0x0020;
// const C_FLAG: u16 = 0x0010;

/// Representation of the Sharp LR35902 used in the GameBoy
/// The CPU contains 6 registers an opcode table and the main memory of the system
/// This struct is responsible for Fetching, Decoding and Executing instructions
pub struct CPU {
    /// The Program Counter (PC) is responisble storing where in "memory" the system is at, it is an index inside the program vector
    pc: u16, //Program Counter
    /// The AF register is 2, u8 bit registers combined into 1, u16 bit register responsible for the Accumulator (High 8 bits) and the CPU flags (Low 8 bits)
    af: u16, //Accumulator&Flags Hi=Accumulator Lo=Flags
    /// The BC register is 2, u8 bit registers combined into 1, u16 bit register where the higher 8 bits are the B register and the lower 8 bits are the C register
    bc: u16, //B&C Hi=B Lo=C
    /// The BC register is 2, u8 bit registers combined into 1, u16 bit register where the higher 8 bits are the D register and the lower 8 bits are the E register
    de: u16, //D&E Hi=D Lo=E
    /// The HL register is 2, u8 bit registers combined into 1, u16 bit register where the higher 8 bits are the H register and the lower 8 bits are the L register
    hl: u16, //H&L Hi=H Lo=L
    /// The SP register is 1, u16 bit register that stores the memory address for the top of the stack, it automatically decrements instelf before pushing something onto the stack and vice versa with popping on the stack
    sp: u16, //Stack Pointer
    /// Memory is a u8 vector that stores the program instructions, it is temporary and is to be replaced with a proper memory map and passed in via cartridge/ROM data
    stop: bool,
    pub bus: BUS,
    opcode_table: [Opcode; 256],
}

type Opcode = fn(&mut CPU);

pub fn wait_for_enter() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}

impl CPU {
    /// Implimentation of CPU struct
    pub fn new(bus: BUS) -> Self {
        CPU {
            pc: 0,
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0xFFFE,
            stop: false,
            bus: bus,
            opcode_table: [CPU::op_null; 256],
        }
    }

    // temp set to pub for testing
    pub fn flip_flag(&mut self, flag: u16) {
        self.af ^= flag;
    }

    fn set_flag(&mut self, flag: u16) {
        self.af |= flag;
    }

    fn clear_flag(&mut self, flag: u16) {
        self.af &= !flag;
    }

    fn check_z(&mut self, data: u8) {
        if data == 0x00 {
            self.set_flag(Z_FLAG);
        } else {
            self.clear_flag(Z_FLAG);
        }
    }

    fn check_hc_add(&mut self, a: u8, b: u8, data: u8) {
        if ((a ^ b ^ data) & 0x10) != 0 {
            self.set_flag(H_FLAG);
        } else {
            self.clear_flag(H_FLAG);
        }
    }

    fn check_hc_sub(&mut self, a: u8, b: u8, data: u8) {
        if (a & 0x0F) < (b & 0x0F) {
            self.set_flag(H_FLAG);
        } else {
            self.clear_flag(H_FLAG);
        }
    }

    fn check_c(&mut self, carried: bool) {
        if carried == true {
            self.set_flag(C_FLAG);
        } else {
            self.clear_flag(C_FLAG);
        }
    }

    fn check_flag_set(&self, flag: u16) -> bool {
        (self.af & flag) != 0
    }

    fn push(&mut self, register: u16) {
        let high = registers::fetch_register_high(register);
        let low = registers::fetch_register_low(register);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write(self.sp, high);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write(self.sp, low);
    }

    fn pop(&mut self) -> u16 {
        let low = self.bus.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let high = self.bus.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        return (high << 8) | low;
    }

    fn fetch_u8(&mut self) -> u8 {
        //let byte = self.memory.read_mem([self.pc as usize]);
        let byte = self.bus.read(self.pc);
        self.pc += 1;
        return byte;
    }

    fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch_u8() as u16;
        let high = self.fetch_u8() as u16;
        return (high << 8) | low;
    }

    fn execute(&mut self, opcode: u8) {
        let handler = self.opcode_table[opcode as usize];
        handler(self);
    }
    /// Run
    pub fn run(&mut self, debug: bool) {
        self.pc = 0x0000;

        if debug {
            while !self.stop {
                let opcode = self.fetch_u8();
                println!(
                    "
                    Opcode: {o:#02X}
                    PC: {pc:#04X} : {pc}
                    AF: {af:#04X} : {af}
                    BC: {bc:#04X} : {bc}
                    DE: {de:#04X} : {de}
                    HL: {hl:#04X} : {hl}
                    SP: {sp:#04X} : {sp}",
                    o = opcode,
                    pc = self.pc,
                    af = self.af,
                    bc = self.bc,
                    de = self.de,
                    hl = self.hl,
                    sp = self.sp
                );
                wait_for_enter();
                //let opcode = self.fetch_u8();
                self.execute(opcode);
            }
        } else {
            while !self.stop {
                let opcode = self.fetch_u8();
                self.execute(opcode);
            }
        }
    }
}
