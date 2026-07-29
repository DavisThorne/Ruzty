use super::CPU;
//use crate::cpu::flags::H_FLAG;
//use crate::cpu::flags::*;
use crate::cpu::registers::fetch_register_high;
use crate::cpu::registers::{self, set_register_high};

#[doc(hidden)]
impl CPU {
    pub fn op_null(&mut self) {
        println!("op_null hit");
        //panic!("Unimplemnted Opcode");
    }
    pub fn op_nop(&mut self) {
        println!("op_nop hit");
        //panic!("Stop code hit");
    }
    pub fn op_stop(&mut self) {
        println!("op_stop hit");
        self.stop = true;
    }
    pub fn op_jr_nz_s8(&mut self) {
        if (self.af & crate::cpu::flags::Z_FLAG) == 0 {
            self.pc += 0x07; // only jump by 7 because cpu.fetch increments PC by 1
        }
    }
    pub fn op_jr_nc_s8(&mut self) {
        if (self.af & crate::cpu::flags::C_FLAG) == 0 {
            self.pc += 0x07;
        }
    }
    pub fn op_ld_b_b(&mut self) {
        let data = registers::fetch_register_high(self.bc);
        registers::set_register_high(&mut self.bc, data);
    }
    pub fn op_ld_d_b(&mut self) {
        let data = registers::fetch_register_high(self.bc);
        registers::set_register_high(&mut self.de, data);
    }
    pub fn op_ld_h_b(&mut self) {
        let data = registers::fetch_register_high(self.bc);
        registers::set_register_high(&mut self.hl, data);
    }
    pub fn op_ld_hl_b(&mut self) {
        let data = registers::fetch_register_high(self.bc);
        self.bus.write(self.hl, data);
    }
    pub fn op_ld_bc_d16(&mut self) {
        //let param = ((self.memory[(self.pc+1) as usize] as u16) << 8) | self.memory[(self.pc) as usize] as u16;
        let param = self.fetch_u16() as u16;
        self.bc = param;
    }

    pub fn op_add_a_b(&mut self) {
        let b = registers::fetch_register_high(self.bc);
        let a = registers::fetch_register_high(self.af);
        let (data, carried) = a.overflowing_add(b);
        self.check_z(data);
        self.check_hc_add(a, b, data);
        self.check_c(carried);
        self.clear_flag(crate::cpu::flags::N_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_sub_b(&mut self) {
        let a = registers::fetch_register_high(self.af);
        let b = registers::fetch_register_high(self.bc);
        let (data, carried) = a.overflowing_sub(b);
        self.check_z(data);
        self.check_hc_sub(a, b, data);
        self.check_c(carried);
        self.set_flag(crate::cpu::flags::N_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_and_a_b(&mut self) {
        let a = registers::fetch_register_high(self.af);
        let b = registers::fetch_register_high(self.bc);
        let data = b & a;
        self.clear_flag(crate::cpu::flags::H_FLAG);
        self.check_z(data);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_or_a_b(&mut self) {
        let a = registers::fetch_register_high(self.af);
        let b = registers::fetch_register_high(self.bc);
        let data = b | a;
        self.check_z(data);
        self.clear_flag(crate::cpu::flags::N_FLAG);
        self.clear_flag(crate::cpu::flags::H_FLAG);
        self.clear_flag(crate::cpu::flags::C_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_ret_nz(&mut self) {
        if !self.check_flag_set(crate::cpu::flags::Z_FLAG) {
            self.pc = self.pop();
        }
    }

    pub fn op_ret_nc(&mut self) {
        if !self.check_flag_set(crate::cpu::flags::C_FLAG) {
            self.pc = self.pop();
        }
    }

    pub fn op_ld_de_d16(&mut self) {
        //let param = ((self.memory.memory[(self.pc+1) as usize] as u16) << 8) | self.memory.memory[(self.pc) as usize] as u16;
        let param = self.fetch_u16() as u16;
        self.de = param;
    }
    pub fn op_ld_hl_d16(&mut self) {
        let param = self.fetch_u16() as u16;
        self.hl = param;
    }
    pub fn op_ld_sp_d16(&mut self) {
        let param = self.fetch_u16() as u16;
        self.sp = param;
    }

    pub fn build_opcode_table(&mut self) {
        self.opcode_table[0x00] = CPU::op_nop;
        self.opcode_table[0x10] = CPU::op_stop;
        self.opcode_table[0x20] = CPU::op_jr_nz_s8;
        self.opcode_table[0x30] = CPU::op_jr_nc_s8;
        self.opcode_table[0x40] = CPU::op_ld_b_b;
        self.opcode_table[0x50] = CPU::op_ld_d_b;
        self.opcode_table[0x60] = CPU::op_ld_h_b;
        self.opcode_table[0x70] = CPU::op_ld_hl_b;
        self.opcode_table[0x80] = CPU::op_add_a_b;
        self.opcode_table[0x90] = CPU::op_sub_b;
        self.opcode_table[0xA0] = CPU::op_and_a_b;
        self.opcode_table[0xB0] = CPU::op_or_a_b;
        self.opcode_table[0xC0] = CPU::op_ret_nz;
        self.opcode_table[0xD0] = CPU::op_ret_nc;
        self.opcode_table[0x01] = CPU::op_ld_bc_d16;
        self.opcode_table[0x11] = CPU::op_ld_de_d16;
        self.opcode_table[0x21] = CPU::op_ld_hl_d16;
        self.opcode_table[0x31] = CPU::op_ld_sp_d16;
        println!("Opcode table Built")
    }
}
