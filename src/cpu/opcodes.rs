use super::CPU;
use super::flags;
use super::registers;

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

    pub fn set_r8(&mut self, dst: registers::R8, src: u8) {
        match dst {
            registers::R8::B => registers::set_register_high(&mut self.bc, src),
            registers::R8::C => registers::set_register_low(&mut self.bc, src),
            registers::R8::D => registers::set_register_high(&mut self.de, src),
            registers::R8::E => registers::set_register_low(&mut self.de, src),
            registers::R8::H => registers::set_register_high(&mut self.hl, src),
            registers::R8::L => registers::set_register_low(&mut self.hl, src),
            registers::R8::HL => self.bus.write(self.hl, src),
            registers::R8::A => registers::set_register_high(&mut self.af, src),
            _ => unreachable!(),
        }
    }

    pub fn get_r16(&mut self, index: registers::R16) -> u16 {
        match index {
            registers::R16::BC => self.bc,
            registers::R16::DE => self.de,
            registers::R16::HL => self.hl,
            registers::R16::SP => self.sp,
            _ => unreachable!(),
        }
    }

    pub fn set_r16(&mut self, index: registers::R16, src: u16) {
        match index {
            registers::R16::BC => self.bc = src,
            registers::R16::DE => self.de = src,
            registers::R16::HL => self.hl = src,
            registers::R16::SP => self.sp = src,
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

    pub fn op_ld_r8_r8(&mut self, dst: registers::R8, src: u8) {
        self.set_r8(dst, src);
    }

    pub fn op_ld_r8_d8(&mut self, dst: registers::R8) {
        let src = self.fetch_u8();
        self.set_r8(dst, src);
    }

    pub fn op_ld_r16_d16(&mut self, dst: registers::R16) {
        let src = self.fetch_u16() as u16;
        self.set_r16(dst, src)
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

    pub fn op_adc_r8(&mut self, src: u8) {
        let a = registers::fetch_register_high(self.af) as u16;
        let carry: u16 = if self.check_flag_set(flags::C_FLAG) {
            1
        } else {
            0
        };
        let sum = a + (src as u16) + carry;
        let data = sum as u8;
        let hc = ((a & 0xF) + ((src as u16) & 0xF) + carry) > 0xF;
        let c = sum > 0xFF;
        self.check_z(data);
        if hc {
            self.set_flag(flags::H_FLAG);
        } else {
            self.clear_flag(flags::H_FLAG);
        }
        if c {
            self.set_flag(flags::C_FLAG);
        } else {
            self.clear_flag(flags::C_FLAG);
        }
        self.clear_flag(flags::N_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_sub_r8(&mut self, src: u8) {
        let a = registers::fetch_register_high(self.af);
        let (data, carried) = a.overflowing_sub(src);
        self.check_c(carried);
        self.check_hc_sub(a, src, data);
        self.check_c(carried);
        self.set_flag(flags::N_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_sbc_r8(&mut self, src: u8) {
        let a = registers::fetch_register_high(self.af) as u16;
        let carry: u16 = if self.check_flag_set(flags::C_FLAG) {
            1
        } else {
            0
        };
        let sum = a.wrapping_sub(src as u16).wrapping_sub(carry);
        let data = sum as u8;
        let hc = ((a as u8) & 0xF) < ((src & 0xF) + (carry as u8));
        let c = a < (src as u16) + (carry as u16);
        if hc {
            self.set_flag(flags::H_FLAG);
        } else {
            self.clear_flag(flags::H_FLAG);
        }
        if c {
            self.set_flag(flags::C_FLAG);
        } else {
            self.clear_flag(flags::C_FLAG);
        }
        self.check_z(data);
        self.set_flag(flags::N_FLAG);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_and_r8(&mut self, src: u8) {
        let a = registers::fetch_register_high(self.af);
        let data = src & a;
        self.clear_flag(flags::H_FLAG);
        self.check_z(data);
        registers::set_register_high(&mut self.af, data);
    }

    pub fn op_or_r8(&mut self, src: u8) {
        let a = registers::fetch_register_high(self.af);
        let data = src | a;
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
}
