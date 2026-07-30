use super::flags;
use super::registers;
use super::CPU;

#[doc(hidden)]
#[derive(Debug)]
pub enum AluOp {
    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
}

pub fn decode_alu_operation(opcode: u8) -> AluOp {
    let index = (opcode & 0b0011_1000) >> 3;
    match index {
        0 => AluOp::ADD,
        1 => AluOp::ADC,
        2 => AluOp::SUB,
        3 => AluOp::SBC,
        4 => AluOp::AND,
        5 => AluOp::XOR,
        6 => AluOp::OR,
        7 => AluOp::CP,
        _ => unreachable!(),
    }
}

impl CPU {
    pub fn get_r8(&mut self, index: registers::R8) -> u8 {
        match index {
            registers::R8::B => registers::fetch_register_high(self.bc),
            registers::R8::C => registers::fetch_register_low(self.bc),
            registers::R8::D => registers::fetch_register_high(self.de),
            registers::R8::E => registers::fetch_register_low(self.de),
            registers::R8::H => registers::fetch_register_high(self.hl),
            registers::R8::L => registers::fetch_register_low(self.hl),
            registers::R8::HL => self.bus.read(self.hl),
            registers::R8::A => registers::fetch_register_high(self.af),
            _ => unreachable!(),
        }
    }

    pub fn set_r8(&mut self, index: registers::R8, data: u8) {
        match index {
            registers::R8::B => registers::set_register_high(&mut self.bc, data),
            registers::R8::C => registers::set_register_low(&mut self.bc, data),
            registers::R8::D => registers::set_register_high(&mut self.de, data),
            registers::R8::E => registers::set_register_low(&mut self.de, data),
            registers::R8::H => registers::set_register_high(&mut self.hl, data),
            registers::R8::L => registers::set_register_low(&mut self.hl, data),
            registers::R8::HL => self.bus.write(self.hl, data),
            registers::R8::A => registers::set_register_high(&mut self.af, data),
            _ => unreachable!(),
        }
    }

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
        if (self.af & flags::Z_FLAG) == 0 {
            self.pc += 0x07; // only jump by 7 because cpu.fetch increments PC by 1
        }
    }
    pub fn op_jr_nc_s8(&mut self) {
        if (self.af & flags::C_FLAG) == 0 {
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

    pub fn op_ld_r8_r8(&mut self, dst: u16, src: u16) {}

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
        self.clear_flag(flags::N_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_add_r8(&mut self, src: u8) {
        let a = registers::fetch_register_high(self.af);
        let (data, carried) = a.overflowing_add(src);
        self.check_z(data);
        self.check_hc_add(a, src, data);
        self.check_c(carried);
        self.clear_flag(flags::N_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_sub_b(&mut self) {
        let a = registers::fetch_register_high(self.af);
        let b = registers::fetch_register_high(self.bc);
        let (data, carried) = a.overflowing_sub(b);
        self.check_z(data);
        self.check_hc_sub(a, b, data);
        self.check_c(carried);
        self.set_flag(flags::N_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_and_a_b(&mut self) {
        let a = registers::fetch_register_high(self.af);
        let b = registers::fetch_register_high(self.bc);
        let data = b & a;
        self.clear_flag(flags::H_FLAG);
        self.check_z(data);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_or_a_b(&mut self) {
        let a = registers::fetch_register_high(self.af);
        let b = registers::fetch_register_high(self.bc);
        let data = b | a;
        self.check_z(data);
        self.clear_flag(flags::N_FLAG);
        self.clear_flag(flags::H_FLAG);
        self.clear_flag(flags::C_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_ret_nz(&mut self) {
        if !self.check_flag_set(flags::Z_FLAG) {
            self.pc = self.pop();
        }
    }

    pub fn op_ret_nc(&mut self) {
        if !self.check_flag_set(flags::C_FLAG) {
            self.pc = self.pop();
        }
    }

    pub fn op_ld_a8_a(&mut self) {
        // a8 refers to address location provided by operand
        let addr = self.fetch_u8();
        self.bus.write(
            0xFF00 + addr as u16,
            registers::fetch_register_high(self.af),
        );
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
