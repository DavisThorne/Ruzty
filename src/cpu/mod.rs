#[doc(inline)]

use std::io;
use crate::bus::BUS;
pub mod flags;

const Z_FLAG: u16 = 0x0080;
const N_FLAG: u16 = 0x0040;
const H_FLAG: u16 = 0x0020;
const C_FLAG: u16 = 0x0010;

/// Representation of the Sharp LR35902 used in the GameBoy
/// The CPU contains 6 registers an opcode table and the main memory of the system
/// This struct is responsible for Fetching, Decoding and Executing instructions
pub struct CPU {
    /// The Program Counter (PC) is responisble storing where in "memory" the system is at, it is an index inside the program vector
    pc: u16,    //Program Counter
    /// The AF register is 2, u8 bit registers combined into 1, u16 bit register responsible for the Accumulator (High 8 bits) and the CPU flags (Low 8 bits)
    af: u16,    //Accumulator&Flags Hi=Accumulator Lo=Flags
    /// The BC register is 2, u8 bit registers combined into 1, u16 bit register where the higher 8 bits are the B register and hte lower 8 bits are the C register
    bc: u16,    //B&C Hi=B Lo=C
    /// The BC register is 2, u8 bit registers combined into 1, u16 bit register where the higher 8 bits are the D register and hte lower 8 bits are the E register
    de: u16,    //D&E Hi=D Lo=E
    /// The BC register is 2, u8 bit registers combined into 1, u16 bit register where the higher 8 bits are the H register and hte lower 8 bits are the L register
    hl: u16,    //H&L Hi=H Lo=L
    /// The SP register is 1, u16 bit register that stores the memory address for the top of the stack, it automatically decrements instelf before pushing something onto the stack and vice versa with popping on the stack
    sp: u16,     //Stack Pointer
    /// Memory is a u8 vector that stores the program instructions, it is temporary and is to be replaced with a proper memory map and passed in via cartridge/ROM data
    stop: bool,
    pub bus: BUS,
    opcode_table: [Opcode; 256]
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
            sp: 0,
            stop: false,
            bus: bus,
            opcode_table: [CPU::op_null; 256]
        }
        //return CPU;
    }

    // temp set to pub for testing
    pub fn flip_flag(&mut self, flag: u16){
        self.af ^= flag;
    }

    fn fetch_u8(&mut self) -> u8 {
        //let byte = self.memory.read_mem([self.pc as usize]);
        let byte = self.bus.read(self.pc);
        self.pc+=1;
        return byte;
    }

    fn fetch_u16(&mut self) -> u16 {
        let low = self.fetch_u8() as u16;
        let high = self.fetch_u8() as u16;
        return (high<<8) | low;
    }

    fn fetch_register_high(&mut self, register: u16) -> u8 {
        let value = (register >> 8) as u8;
        return value;
    }

    fn fetch_register_low(&mut self, register: u16) -> u8 {
        let value = (register & 0x00FF) as u8;
        return value;
    }

    fn set_register_high(&mut self, register: &mut u16, data: u8) {
        *register = (*register & 0x00FF) | (data as u16) << 8;
    }

    fn set_register_low(&mut self, register: &mut u16, data: u8) {
        *register = (*register & 0xFF00) | (data as u16);
    }

    fn execute(&mut self, opcode: u8) {
        let handler = self.opcode_table[opcode as usize];
        handler(self);
    }
    /// Run
    pub fn run(&mut self, debug: bool){
        self.pc = 0x0000;

        if debug {
            while !self.stop {
                let opcode = self.fetch_u8();
                println!("
                    Opcode: 0x{:02X}
                    PC:0x{:04X}
                    AF:{}
                    BC:{}
                    DE:{}
                    HL:{}
                    SP{}",
                    opcode,self.pc,self.af,(self.bc),self.de,self.hl,self.sp);
                wait_for_enter(); 
                //let opcode = self.fetch_u8();
                self.execute(opcode);        
            }
        }
        else {
            while !self.stop {
                let opcode = self.fetch_u8();
                self.execute(opcode);         
            }
        }
    }
}

mod opcodes;